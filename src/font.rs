//! The font rasterization
//!
//! The glyph painting is a set of commands defined by [GlyphStep]. Every
//! [GlyphStep] can be packed to 12 bits, it contains the coordinates
//! and a specific command.
//!
//! The [`draw_glyph`] function is a common way to draw a character.
//!
//! The set of [`GlyphTable`]s is a common way to define a symbol table.
//! If the `font_data` feature is enabled, a standard table may be found
//! in [`crate::font_data::TABLES`](font_data/static.TABLES.html) constant
//! There is a [gui tool](https://github.com/disiamylborane/draw-i16-fontviewer)
//! to create or modify a glyph table.

use ranged_integers::*;
use crate::{v2, Drawable, V2};

/// Part of glyph drawing step: the current action
#[derive(Clone, PartialEq, Eq)]
pub enum GlyphConnectionType {
    /// Jump to a new location
    Break,
    /// Draw line from previous location
    Outline{
        /// Thicker or thinner line
        thick: bool,
        /// Draw the next line from the current (true) or previous (false) position
        update: bool
    },
    /// Alter the command for the next point
    /// 
    /// - Control+Outline = Spline, draw a quadratic spline, use this as the control point between the previous and the next
    /// - Control+Break = Recall, draw another symbol using [recall_to_code()] function to get a character code
    Control,
    /// Quarter Circle
    Oval {
        /// Paint halfcircle starting inclining right or left from straight line
        right : bool
    },
}

/// Coordinates used in GlyphStep
#[derive(Clone,Copy,PartialEq, Eq)]
pub struct GlyphCoord {
    /// X coordinate
    pub x : Ranged<0,15>,
    /// Y coordinate
    pub y : Ranged<0,31>,
}

/// One step of symbol drawing, contains of action and coordinates
#[derive(Clone)]
pub struct GlyphStep {
    /// Current point to apply action to
    pub coord : GlyphCoord,
    /// Current action (aka type)
    pub tp : GlyphConnectionType
}

/// Packed data containing some unicode block. `&[GlyphTable]` implements `GlyphProvider`.
pub struct GlyphTable {
    /// Code of the first char in the block (count is specified by addr.len())
    pub basechar: char,
    /// Adresses of the beginnings of chars
    pub addr: &'static [u16],
    /// Compacted GlyphSteps (1 step per 12 bits)
    pub data: &'static [[u8; 3]],
}

/// Abstraction for the GlyphStep generators
pub trait GlyphProvider : Copy {
    /// Yield the set of steps for the specified char
    fn get_glyph(self, char: u32) -> Option<impl Iterator<Item=GlyphStep>>;
}

/// Glyph provider that provides no glyphs
///
/// Can be used as a dummy instance of GlyphProvider, is used in the
/// [`Recall`](GlyphConnectionType::Control) command handling
#[derive(Clone, Copy)]
pub struct EmptyGlyphProvider;
impl GlyphProvider for EmptyGlyphProvider {
    fn get_glyph(self, _ch: u32) -> Option<impl Iterator<Item=GlyphStep>> { Option::<core::iter::Cloned<core::slice::Iter<'_, GlyphStep>>>::None }
}

impl GlyphProvider for &[GlyphTable] {
    fn get_glyph(self, ch: u32) -> Option<impl Iterator<Item=GlyphStep>> {
        struct GlyphIter<'a> {s: &'a [[u8; 3]], end: usize, current: usize}
        impl Iterator for GlyphIter<'_>{
            type Item = GlyphStep;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current != self.end {
                    let step = get_glyphstep(self.s, self.current);
                    self.current += 1;
                    Some(step)
                }
                else {None}
            }
        }

        for tbl in self {
            let idx = ch as usize - tbl.basechar as usize;
            if idx >= tbl.addr.len() {
                continue;
            }
            let end = if idx+1 == tbl.addr.len()
                                {tbl.data.len()*2}
                        else {tbl.addr[idx+1] as usize};

            let ops = tbl.addr[idx] as usize .. end;
            return Some(GlyphIter{s:tbl.data, end: ops.end, current: ops.start});
        }

        None
    }
}

impl GlyphConnectionType {
    const fn from_raw(raw: Ranged<0, 0b111>) -> Self {
        rmatch!{[0 0b111] raw
            0b000 => {Self::Break}
            0b001 => {Self::Control}
            0b010 => {Self::Oval{right: false}}
            0b011 => {Self::Oval{right: true}}
            0b100 => {Self::Outline{thick: false, update: false}}
            0b101 => {Self::Outline{thick: false, update: true}}
            0b110 => {Self::Outline{thick: true, update: false}}
            0b111 => {Self::Outline{thick: true, update: true}}
        }
    }
    const fn into_raw(self) -> Ranged<0, 0b111> {
        match self {
            GlyphConnectionType::Break => r!([] 0b000),
            GlyphConnectionType::Control => r!([] 0b001),
            GlyphConnectionType::Oval{right: false} => r!([] 0b010),
            GlyphConnectionType::Oval{right: true} => r!([] 0b011),
            Self::Outline{thick: false, update: false} => r!([] 0b100),
            Self::Outline{thick: false, update: true} => r!([] 0b101),
            Self::Outline{thick: true, update: false} => r!([] 0b110),
            Self::Outline{thick: true, update: true} => r!([] 0b111),
        }
    }
}

impl GlyphCoord {
    fn gap_by_xsize(xsize: u8) -> u8 {
        let gap = xsize/5;
        if (xsize-gap) % 2 == 0 {gap+1}
        else {gap}
    }

    fn get_real_coords(self, glyphsize: V2) -> V2 {
        let xgap = Self::gap_by_xsize(glyphsize.x as u8) as i16;
        let xglyph = glyphsize.x - xgap;
        let cx = self.x.i16();
        let x = match cx {
            0 ..= 5 => cx * xglyph/12,
            6 => xglyph/2,
            7 ..= 12 => (xglyph-1) - (12-cx)*(xglyph-1)/12,
            13 ..= 15 => xglyph + (cx-13)*xgap/3,
            _ => unreachable!(),
        } + xgap/2;

        let y = self.y.i16() * glyphsize.y / 31;

        V2{x, y}
    }
}

impl GlyphStep {
    /// Convert a raw 12-bit number to GlyphStep
    pub fn from_raw(raw: Ranged<0, 0xFFF>) -> Self {
        use core::ops::Div;
        use core::ops::Rem;

        let rtp = raw.div(r!(512));
        let y = (raw.div(r!(16))).rem(r!(32));
        let x = raw.rem(r!(16));

        Self {
            coord: GlyphCoord{x, y},
            tp: GlyphConnectionType::from_raw(rtp)
        }
    }

    /// Convert to a 12-bit representation, may be converted back by `from_raw`
    pub const fn into_raw(self) -> u16 {
        let rtp = self.tp.into_raw().u16();
        let ryv = self.coord.y.u16();
        let rxv = self.coord.x.u16();
        rtp * 512 + ryv * 16 + rxv
    }

    /// Convert a pair of steps to raw data
    pub const fn to_gdata(s1: Self, s2: Self) -> [u8; 3] {
        let s1 = s1.into_raw();
        let s2 = s2.into_raw();
        [(s1 & 0xFF) as u8, ((s1 & 0xF00) >> 8) as u8 | ((s2 & 0x0F) << 4) as u8,  ((s2 & 0xFF0)>>4) as u8]
    }

    /// Convert raw data to a pair of steps, panics on invalid input
    pub fn from_gdata(gd: [u8; 3]) -> [Self;2] {
        let s1 = gd[0].as_ranged() + gd[1] % r!(0b10000) * r!(256);
        let s2 = gd[1].as_ranged() / r!(0b10000) + gd[2].as_ranged() * r!(0b10000);
        [Self::from_raw(s1), Self::from_raw(s2)]
    }
}

/// Get a font rect size the library is trimmed for
pub const fn fontsize_to_glyphsize(fsize: Ranged<6,63>)-> V2 {
    let fsize = fsize.i16()-5;
    let gap = (fsize + 3) / 4;
    let glsize = fsize+3 + (fsize%2);

    V2::new(gap+glsize, fsize+(fsize*2 + 3))
}


fn get_glyphstep(data: &[[u8; 3]], item: usize) -> GlyphStep {
    let cell = data[item/2];
    GlyphStep::from_gdata(cell)[item % 2].clone()
}


impl GlyphTable {
    /// Get GlyphStep from table by index
    pub fn get_glyphstep(&self, item: usize) -> GlyphStep {
        get_glyphstep(self.data, item)
    }
}

/// Get the glyph to be recalled by a pair of 'Recall' step and the step next to it
pub fn recall_to_code(fst: GlyphCoord, snd: GlyphCoord) -> Ranged<0, 0x3ffff> {
    let code: Ranged<0, 15> = fst.x;
    let code: Ranged<0, 511> = code + fst.y*r!(16);
    let code: Ranged<0, 8191> = code + snd.x * r!(512);
    let code: Ranged<0, 0x3ffff> = code + snd.y * r!(8192);
    code
}
/// Encode the character to recall as a pair of Recall type GlyphSteps
///
/// The code may be decoded by [`recall_to_code`].
pub fn code_to_recall(code: Ranged<0, 0x3ffff>) -> (GlyphCoord, GlyphCoord) {
    let fstx = code % r!(16);
    let fsty = code % r!(512) / r!(16);
    let sndx = code % r!(8192) / r!(512);
    let sndy = code / r!(8192);
    (GlyphCoord{x: fstx, y:fsty},GlyphCoord{x: sndx, y:sndy})
}

/// Draw a glyph on Drawable.
///
/// Glyph defined by `steps` is drawn on the `drawable`. The function uses the whole
/// [drawable size](crate::Drawable::_size) as a font size, so, typically applying
/// of [`Stencil`](crate::Stencil) is needed. For the [Recall](GlyphConnectionType::Control)
/// steps the glyph will be searched in the `recall_tables`, consider passing
/// `&[GlyphTable]` or `EmptyGlyphProvider`.
pub fn draw_glyph<Colour:Copy>(steps: impl Iterator<Item=GlyphStep>, drawable: &mut dyn Drawable<Colour>, colour: Colour, recall_tables: impl GlyphProvider) {
    let mut prevpoint: GlyphCoord = GlyphCoord{x: r!([] 0), y: r!([] 2)};
    let mut ctrlpoint: Option<GlyphCoord> = None;

    let csize = drawable.size();

    for GlyphStep { coord, tp } in steps {
        match tp {
            GlyphConnectionType::Control => {
                ctrlpoint = Some(coord);
            },
            GlyphConnectionType::Break => {
                if let Some(cp) = ctrlpoint {
                    let char = recall_to_code(cp, coord);
                    if let Some(iter) = recall_tables.get_glyph(char.u32()) {
                        draw_glyph(iter, drawable, colour, EmptyGlyphProvider);
                    }
                }
                else {
                    prevpoint = coord;
                }
                ctrlpoint = None;
            }
            GlyphConnectionType::Outline { thick, update } => {
                let width = line_width(thick, csize);
                
                if let Some(cp) = ctrlpoint {
                    drawable.quad_spline(prevpoint.get_real_coords(csize), cp.get_real_coords(csize), coord.get_real_coords(csize), colour, width);
                }
                else {
                    drawable.line(prevpoint.get_real_coords(csize), coord.get_real_coords(csize), colour, width);
                }
                if update {prevpoint = coord;}
                ctrlpoint = None;
            }
            GlyphConnectionType::Oval{right} => {
                let width = line_width(true, csize);
                let prev = prevpoint.get_real_coords(csize);
                let curr = coord.get_real_coords(csize);

                if let Some(_control) = ctrlpoint.map(|cp| cp.get_real_coords(csize)) {
                    // Nothing for now
                }
                else {
                    let center = if ((curr.x >= prev.x) == (curr.y >= prev.y)) != right {v2(prev.x, curr.y)} else {v2(curr.x, prev.y)};
                    let radii = curr-prev;

                    if right {
                        drawable.ellipse_at_center(center, (radii.x.abs(), radii.y.abs()), colour, [
                            curr.y > prev.y && curr.x <= prev.x,
                            curr.y <= prev.y && curr.x <= prev.x,
                            curr.y > prev.y && curr.x > prev.x,
                            curr.y <= prev.y && curr.x > prev.x,
                        ], width);
                    }
                    else {
                        drawable.ellipse_at_center(center, (radii.x.abs(), radii.y.abs()), colour, [
                            curr.y <= prev.y && curr.x > prev.x,
                            curr.y >  prev.y && curr.x > prev.x,
                            curr.y <= prev.y && curr.x <= prev.x,
                            curr.y > prev.y && curr.x <= prev.x,
                        ], width);
                    }
                }

                prevpoint = coord;
                ctrlpoint = None;   
            }
        };
    }
}

fn line_width(thick: bool, glyphsize: V2) -> u8 {
    let mainwidth = 1 + (glyphsize.x/16) as u8;
    if thick {mainwidth} else {(mainwidth + 1) / 2}
}
