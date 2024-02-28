
use core::ops::Range;
use super::*;


pub(super) fn draw_unknown(canvas: &mut dyn Drawable, colour: Colour) {
    let steps = [
        GlyphStep{coord: GlyphCoord{x: r!([] 0), y: r!([] 24)}, 
                tp: GlyphConnectionType::Outline{thick: false, update: true}},
        GlyphStep{coord: GlyphCoord{x: r!([] 12), y: r!([] 24)}, 
                tp: GlyphConnectionType::Outline{thick: false, update: true}},
        GlyphStep{coord: GlyphCoord{x: r!([] 12), y: r!([] 0)}, 
                tp: GlyphConnectionType::Outline{thick: false, update: true}},
        GlyphStep{coord: GlyphCoord{x: r!([] 0), y: r!([] 0)},
                tp: GlyphConnectionType::Outline{thick: false, update: true}},
    ];

    let mut iter = steps.iter().cloned();

    draw_glyph(&mut iter, canvas, colour)
}


pub(super) fn draw_char(ch: char, canvas: &mut dyn Drawable, colour: Colour) {
    for tab in font_data::TABLES.iter() {
        if tab.try_draw_glyph(canvas, ch, colour).is_ok() {
            return;
        }
    }

    draw_unknown(canvas, colour)
}


#[derive(Clone)]
pub(super) enum GlyphConnectionType {
    Break,
    Outline{thick: bool, update: bool},
    Control
}
impl GlyphConnectionType {
    fn from_raw(raw: u8) -> Self {
        match raw {
            0b000 => Self::Break,
            0b001 => Self::Control,
            0b100 => Self::Outline{thick: false, update: false},
            0b101 => Self::Outline{thick: false, update: true},
            0b110 => Self::Outline{thick: true, update: false},
            0b111 => Self::Outline{thick: true, update: true},
            _ => unreachable!()
        }
    }
    const fn into_raw(self) -> u16 {
        match self {
            Self::Break => 0b000,
            Self::Control => 0b001,
            Self::Outline{thick, update} => 0b100 | (thick as u16)<< 1 | (update as u16),
        }
    }
}

#[derive(Clone,Copy)]
pub(super) struct GlyphCoord {
    pub x : Ranged<0,15>,
    pub y : Ranged<0,31>,
}


#[derive(Clone)]
pub(super) struct GlyphStep {
    pub coord : GlyphCoord,
    pub tp : GlyphConnectionType
}

impl GlyphStep {
    pub(super) fn from_raw(raw: u16) -> Self {
        let rtp = raw.as_ranged()/r!(512);
        let ryv = (raw>>4) % r!(32);
        let rxv = raw % r!(16);

        Self{coord: GlyphCoord{x: rxv, 
                                y: ryv}, 
                tp: GlyphConnectionType::from_raw(rtp.u8())}
    }

    pub(super) const fn into_raw(self) -> u16 {
        let rtp = self.tp.into_raw();
        let ryv = self.coord.y.u16();
        let rxv = self.coord.x.u16();
        rtp<<9 | ryv<<4 | rxv
    }

    pub(super) const fn to_gdata(s1: Self, s2: Self) -> [u8; 3] {
        let s1 = s1.into_raw();
        let s2 = s2.into_raw();
        [(s1 & 0xFF) as u8, ((s1 & 0xF00) >> 8) as u8 | ((s2 & 0x0F) << 4) as u8,  ((s2 & 0xFF0)>>4) as u8]
    }

    pub(super) fn from_gdata(gd: [u8; 3]) -> [Self;2] {
        let s1 = gd[0] as u16 | (((gd[1]&0b1111) as u16) << 8);
        let s2 = ((gd[1] as u16 & 0b11110000) >> 4) | ((gd[2] as u16) << 4);
        [Self::from_raw(s1), Self::from_raw(s2)]
    }
}

pub(super) fn gap_by_xsize(xsize: u8) -> u8 {
    let gap = xsize/5;
    if (xsize-gap) % 2 == 0 {gap+1}
    else {gap}
}

/// Get a font rect size the library is trimmed for
pub fn _glyphsize_by_covnent(fsize: Ranged<6,63>)-> V2 {
    let fsize = fsize.i16()-5;
    let gap = (fsize + 3) / 4;
    let glsize = fsize+3 + (fsize%2);
    
    V2::new(gap+glsize, fsize+(fsize*2 + 3))
}


pub(super) fn draw_glyph(steps: &mut dyn Iterator<Item=GlyphStep>, canvas: &mut dyn Drawable, colour: Colour) {

    let mut ctrlpoint: GlyphCoord = GlyphCoord{x: r!([] 0), y: r!([] 1)};
    let mut prevpoint: GlyphCoord = GlyphCoord{x: r!([] 0), y: r!([] 1)};
    let mut curve_flag = false;

    for step in steps {
        match step.tp {
            GlyphConnectionType::Break => {
                prevpoint = step.coord;
                curve_flag = false;
            },
            GlyphConnectionType::Control => {
                ctrlpoint = step.coord;
                curve_flag = true;
            },
            GlyphConnectionType::Outline{thick, update} => {
                let csize = canvas.size();
                let width = GlyphData::line_width(thick, csize);

                if curve_flag {
                    canvas.quad_bezier(GlyphData::coords(prevpoint, csize), GlyphData::coords(ctrlpoint, csize), GlyphData::coords(step.coord, csize), colour, width);
                }
                else {
                    canvas.line(GlyphData::coords(prevpoint, csize), GlyphData::coords(step.coord, csize), colour, width);
                }

                if update {prevpoint = step.coord;}

                curve_flag = false;
            },
        };

    }
}



pub struct GlyphData(pub(super) &'static [[u8; 3]]);
impl GlyphData {
    fn get(&self, item: usize) -> GlyphStep {
        let cell = self.0[item/2];
        GlyphStep::from_gdata(cell)[item % 2].clone()
    }

    fn line_width(th: bool, glyphsize: V2) -> u8 {
        let mainwidth = 1 + (glyphsize.x/16) as u8;
        let addwidth = (mainwidth + 1) / 2;

        if th {mainwidth} else {addwidth}
    }

    fn coords(c: GlyphCoord, glyphsize: V2) -> V2 {
        let xgap = gap_by_xsize(glyphsize.x as u8) as i16;
        let xglyph = glyphsize.x - xgap;
        let cx = c.x.i16();
        let x = match cx {
            0 ..= 5 => cx * xglyph/12,
            6 => xglyph/2,
            7 ..= 12 => (xglyph-1) - (12-cx)*(xglyph-1)/12,
            13 ..= 15 => xglyph + (cx-13)*xgap/3,
            _ => unreachable!(),
        } + xgap/2;

        let y = c.y.i16() * glyphsize.y / 31;

        V2{x, y}
    }

    
    fn draw_glyph(&self, canvas: &mut dyn Drawable, ops: Range<usize>, colour: Colour) {
        struct GlyphIter<'a> {s: &'a GlyphData, end: usize, current: usize}
        impl<'a> Iterator for GlyphIter<'a>{
            type Item = GlyphStep;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current != self.end {
                    let step = self.s.get(self.current);
                    self.current += 1;
                    Some(step)
                }
                else {None}
            }
        }

        draw_glyph(&mut GlyphIter{s:self, end: ops.end, current: ops.start}, canvas, colour);
    }
}


pub struct GlyphTable {
    pub basechar: char,
    pub addr: &'static [u16],
    pub data: GlyphData,
}
impl GlyphTable {
    pub fn try_draw_glyph(&self, canvas: &mut dyn Drawable, ch: char, colour: Colour)->Result<(),()> {
        let idx = ch as usize - self.basechar as usize;
        if idx >= self.addr.len() {
            return Err(())
        }
        let end = if idx+1 == self.addr.len()
                            {self.data.0.len()*2}
                    else {self.addr[idx+1] as usize};

        let ops = self.addr[idx] as usize .. end;
        self.data.draw_glyph(canvas, ops, colour);
        Ok(())
    }
}



