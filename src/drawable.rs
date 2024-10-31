//! A bunch of rasterizing algorithms

use ranged_integers::r;

use crate::font::GlyphProvider;
use crate::{V2, v2};

#[cfg(any(feature="font_data", doc))]
use crate::{font_data, font::GlyphTable};

use core::cmp::{min, max};

/// A Drawable is an object one can paint something on
///
/// The trait consists of basic drawing functions including setting pixels,
/// horizontal/vertical lines and filled rectangle. It also has the size, and
/// accepts to draw any pixel in the range between 0,0 (inclusively) and the
/// size it has (exclusively).
///
/// Implement the trait and then use `&mut dyn Drawable` to perform the painting.
pub trait Drawable<Colour:Copy> {
    /// Get the drawing area size
    fn _size(&self) -> V2;

    /// Paint the whole drawing area with a `colour`
    fn _clear(&mut self, colour: Colour);

    /// Set a pixel colour
    ///
    /// ### Safety
    /// pos is guaranteed to be inside self._size()
    unsafe fn _pixel(&mut self, pos: V2, colour: Colour);

    /// Draw a horizontal line of `len` pixels length, starting from the `pos` point
    ///
    /// ### Safety
    /// The line must be inside the rectangle `[(0,0), (self._size - (1,1))]`
    unsafe fn _hline(&mut self, pos: V2, len: u16, colour: Colour){
        for x in pos.x..(pos.x+len as i16) {
            self._pixel(v2(x, pos.y), colour);
        }
    }

    /// Draw a vertical line of `len` pixels length, starting from the `pos` point
    ///
    /// ### Safety
    /// The line must be inside the rectangle `[(0,0), (self._size - (1,1))]`
    unsafe fn _vline(&mut self, pos: V2, len: u16, colour: Colour) {
        for y in pos.y..(pos.y+len as i16) {
            self._pixel(v2(pos.x, y), colour);
        }
    }

    /// Draw a filled rectangle [`p1`, `p2`].
    ///
    /// ### Safety
    /// The points are guaranteed to satisfy the following:
    /// * `p1` and `p2` both inside the rectangle `[(0,0), (self._size - (1,1))]`
    /// * `p1.x <= p2.x`
    /// * `p1.y <= p2.y`
    unsafe fn _rect(&mut self, p1: V2, p2: V2, colour: Colour){
        for x in p1.x..(p2.x+1) {
            for y in p1.y..(p2.y+1) {
                self._pixel(v2(x,y), colour);
            }
        }
    }
}

/// A middleware extension for [`Drawable`]s, allowing
/// high-level painting and rasterizing
pub trait DrawableMethods<Colour:Copy>: Drawable<Colour>+Sized {
    /// Get drawable size
    #[inline] fn size(&self) -> V2 { <dyn Drawable<Colour>>::size(self) }

    /// Fill the whole drawable with a colour
    #[inline] fn clear(&mut self, colour: Colour) { <dyn Drawable<Colour>>::clear(self, colour) }

    /// Set a single pixel colour
    #[inline] fn pixel(&mut self, pos: V2, colour: Colour) { <dyn Drawable<Colour>>::pixel(self, pos, colour) }

    /// Paint a thick pixel (rectangle) at a point
    #[inline] fn thick_pixel(&mut self, pos: V2, colour: Colour, width: u8) { <dyn Drawable<Colour>>::thick_pixel(self, pos, colour, width) }

    /// Paint a line
    #[inline] fn line(&mut self, p1: V2, p2: V2, colour: Colour, width: u8) { <dyn Drawable<Colour>>::line(self,p1,p2,colour,width) }

    /// Paint a rectangle contour
    #[inline] fn rect(&mut self, p1: V2, p2: V2, colour: Colour) { <dyn Drawable<Colour>>::rect(self, p1, p2, colour) }

    /// Paint a filled rectangle
    #[inline] fn rect_fill(&mut self, p1: V2, p2: V2, colour: Colour) { <dyn Drawable<Colour>>::rect_fill(self, p1, p2, colour) }

    /// Paint an ellipse contour by center and horizontal/vertical radius
    #[inline] fn ellipse_at_center(&mut self, center: V2, radii: (i16, i16), colour: Colour, quadrants: [bool; 4], width: u8) {
        <dyn Drawable<Colour>>::ellipse_at_center(self, center, radii, colour, quadrants, width);
    }

    /// Paint an ellipse contour by the corner points of the bounding rectangle
    #[inline] fn ellipse_at_rect(&mut self, p1: V2, p2: V2, colour: Colour, quadrants: [bool; 4], width: u8) {
        <dyn Drawable<Colour>>::ellipse_at_rect(self, p1, p2, colour, quadrants, width);
    }

    /// Paint a quadratic bezier curve
    /// 
    /// `p0` and `p2` for line endings, p1 as control point
    #[inline] fn quad_bezier(&mut self, p0: V2, p1: V2, p2: V2, colour: Colour, width: u8) {
        <dyn Drawable<Colour>>::quad_bezier(self, p0, p1, p2, colour, width);
    }

    /// Paint a quadratic spline between 3 points
    /// 
    /// `p0` and `p2` for line endings, p1 as central point
    #[inline] fn quad_spline(&mut self, p0: V2, p1: V2, p2: V2, colour: Colour, width: u8) {
        <dyn Drawable<Colour>>::quad_spline(self, p0, p1, p2, colour, width);
    }

    /// Paint a glyph using the user-defined char code to glyph converter.
    /// 
    /// * `tables` - the code-to-glyph steps converter (see [`GlyphProvider`])
    /// * `ch` - char to display
    /// * `fontsize` - size of one char (consider using [`fontsize_to_glyphsize`](crate::font::fontsize_to_glyphsize) output)
    /// * `pos` - location of top left corner of the character
    /// * `colour` - parameter to draw the symbol pixels with
    #[inline] fn symbol_with_provider<G:GlyphProvider>(&mut self, tables: G, ch: char, fontsize: V2, pos: V2, colour: Colour) {
        <dyn Drawable<Colour>>::symbol_with_provider(self, tables, ch, fontsize, pos, colour);
    }

    /// Paint a line of text using the user-defined char code to glyph converter
    /// 
    /// * `tables` - the code-to-glyph steps converter (see [`GlyphProvider`])
    /// * `s` - string to display
    /// * `fontsize` - size of one char (consider using [`fontsize_to_glyphsize`](crate::font::fontsize_to_glyphsize) output)
    /// * `pos` - location of top left corner of the character
    /// * `colour` - parameter to draw the symbol pixels with
    #[inline] fn text_with_provider<G:GlyphProvider>(&mut self, tables: G, s: &str, fontsize: V2, pos: V2, colour: Colour) {
        <dyn Drawable<Colour>>::text_with_provider(self, tables, s, fontsize, pos, colour);
    }

    /// Paint a glyph using the builtin font
    /// 
    /// * `tables` - the code-to-glyph steps converter (see [`GlyphProvider`])
    /// * `ch` - char to display
    /// * `pos` - location of top left corner of the character
    /// * `colour` - parameter to draw the symbol pixels with
    #[cfg(any(feature="font_data", doc))]
    #[doc(cfg(feature="font_data"))]
    #[inline]
    fn symbol(&mut self, ch: char, fontsize: V2, pos: V2, colour: Colour) {
        <dyn Drawable<Colour>>::symbol(self, ch, fontsize, pos, colour)
    }

    /// Paint a line of text using the builtin font
    /// 
    /// * `tables` - the code-to-glyph steps converter (see [`GlyphProvider`])
    /// * `s` - string to display
    /// * `pos` - location of top left corner of the character
    /// * `colour` - parameter to draw the symbol pixels with
    #[cfg(any(feature="font_data", doc))]
    #[doc(cfg(feature="font_data"))]
    #[inline]
    fn text(&mut self, s: &str, fontsize: V2, pos: V2, colour: Colour) {
        <dyn Drawable<Colour>>::text(self, s, fontsize, pos, colour)
    }
}

impl<T, Colour:Copy> DrawableMethods<Colour> for T where T: Drawable<Colour> {}

impl<Colour:Copy> dyn Drawable<Colour>+'_ {
    /// Get a drawable size
    pub fn size(&self) -> V2 { self._size() }

    /// Fill the whole canvas with a signle colour
    pub fn clear(&mut self, colour: Colour) { self._clear(colour) }

    /// Draw a pixel, if inside the canvas
    /// Does nothing, if outside
    pub fn pixel(&mut self, pos: V2, colour: Colour) {
        let size = self.size();

        if pos.x >= size.x || pos.y >= size.y
            {return;}
        if pos.x < 0 || pos.y < 0
            {return;}

        unsafe{
            self._pixel(pos, colour);
        }
    }

    /// Draw a rectangle at a point
    pub fn thick_pixel(&mut self, pos: V2, colour: Colour, width: u8) {
        if width == 0 {return;}

        let prenmsz = V2::new((width-1) as i16/2, (width-1) as i16/2);
        let prenasz = V2::new(width as i16/2, width as i16/2);

        self.rect_fill(pos-prenmsz, pos+prenasz, colour);
    }

    /// Draw a horizontal line
    /// 
    /// `pos` is a point of line start, `len` is a line length
    pub fn horz_line(&mut self, pos: V2, len: u16, colour: Colour) {
        let mut pos = pos;
        let mut len = len as i16;

        let size = self.size();

        if pos.y >= size.y || pos.y < 0
            {return;}

        if pos.x < 0 {
            if -pos.x > len {return;}
            len -= -pos.x;
            pos.x = 0;
        }
        if pos.x + len >= size.x {
            if pos.x > size.x {return;}
            len = size.x-pos.x;
        }

        unsafe {
            self._hline(pos, len as u16, colour);
        }
    }

    /// Draw a vertical line
    /// 
    /// `pos` is a point of line start, `len` is a line length
    pub fn vert_line(&mut self, pos: V2, len: u16, colour: Colour) {
        let mut npos = pos;
        let mut nlen = len as i16;

        let size = self.size();

        if npos.x >= size.x || npos.x < 0
            {return;}

        if npos.y < 0 {
            if -npos.y > nlen {return;}
            nlen -= -npos.y;
            npos.y = 0;
        }
        if npos.y + nlen >= size.y {
            if npos.y > size.y {return;}
            nlen = size.y-npos.y;
        }

        unsafe {
            self._vline(npos, nlen as u16, colour);
        }
    }

    /// Draw a line
    /// 
    /// `p1` and `p2` are line endings
    pub fn line(&mut self, p1: V2, p2: V2, colour: Colour, width: u8) {
        if width == 0 {return;}

        if p1.y == p2.y {
            for y in (p1.y - (width as i16-1)/2)..(p1.y + 1 + (width as i16)/2) {
                self.horz_line(V2::new(min(p1.x, p2.x), y), (p1.x-p2.x).unsigned_abs()+1, colour);
            }
        }
        else if p1.x == p2.x {
            for x in (p1.x - (width as i16-1)/2)..(p1.x + 1 + (width as i16)/2) {
                self.vert_line(V2::new(x, min(p1.y, p2.y)), (p1.y-p2.y).unsigned_abs()+1, colour);
            }
        }
        else {
            let (x1, x2, y1, y2, change_xy) = {
                let (mut x1, mut x2, mut y1, mut y2) = (p1.x, p2.x, p1.y, p2.y);

                let change_xy = (y2-y1).abs() > (x2-x1).abs();
                if change_xy {
                    core::mem::swap(&mut x1, &mut y1);
                    core::mem::swap(&mut x2, &mut y2);
                }
                if x1 > x2 {
                    core::mem::swap(&mut x1, &mut x2);
                    core::mem::swap(&mut y1, &mut y2);
                }

                (x1, x2, y1, y2, change_xy)
            };

            let dx = x2 - x1; // Positive
            let dy = (y2 - y1).abs();
            let yinc = if y2 > y1 {1} else {-1};

            let mut y = y1;
            let mut err = 0;

            let mut yz = y2;

            let mut recalc_y_err = |y: &mut i16, yz: &mut i16| {
                err += dy;
                if err > dx/2 {
                    err -= dx;
                    *y += yinc;
                    *yz -= yinc;
                }
            };

            let prenmsz = V2::new((width-1) as i16/2, (width-1) as i16/2);
            let prenasz = V2::new(width as i16/2, width as i16/2);

            if change_xy {
                for x in x1..(x1 + (x2-x1)/2 + 1) {
                    let pren1 = V2::new(x, y);
                    let pren2 = V2::new(x2 - (x-x1), yz);
                    self.rect_fill((pren1-prenmsz).swap(), (pren1+prenasz).swap(), colour);
                    self.rect_fill((pren2-prenmsz).swap(), (pren2+prenasz).swap(), colour);
                    recalc_y_err(&mut y, &mut yz);
                }
            }
            else {
                for x in x1..(x1 + (x2-x1)/2 + 1) {
                    let pren1 = V2::new(x, y);
                    let pren2 = V2::new(x2 - (x-x1), yz);
                    self.rect_fill(pren1-prenmsz, pren1+prenasz, colour);
                    self.rect_fill(pren2-prenmsz, pren2+prenasz, colour);
                    recalc_y_err(&mut y, &mut yz);
                }
            }
        }
    }

    /// Draw a rectangle contour
    pub fn rect(&mut self, p1: V2, p2: V2, colour: Colour){
        let x1 = min(p1.x, p2.x);
        let x2 = max(p1.x, p2.x);
        let y1 = min(p1.y, p2.y);
        let y2 = max(p1.y, p2.y);

        self.horz_line(V2::new(x1, y1), (x2-x1+1) as u16, colour);
        self.horz_line(V2::new(x1, y2), (x2-x1+1) as u16, colour);

        self.vert_line(V2::new(x1, y1), (y2-y1+1) as u16, colour);
        self.vert_line(V2::new(x2, y1), (y2-y1+1) as u16, colour);
    }

    /// Draw a filled rectangle
    pub fn rect_fill(&mut self, p1: V2, p2: V2, colour: Colour){
        let size = self.size();

        let mut x1 = min(p1.x, p2.x);
        let mut x2 = max(p1.x, p2.x);
        let mut y1 = min(p1.y, p2.y);
        let mut y2 = max(p1.y, p2.y);

        if x1 >= size.x {return;}
        if x2 < 0 {return;}
        if y1 >= size.y {return;}
        if y2 < 0 {return;}

        if x1 < 0 {x1 = 0;}
        if x2 >= size.x {x2 = size.x-1;}
        if y1 < 0 {y1 = 0;}
        if y2 >= size.y {y2 = size.y-1;}

        unsafe {
            self._rect(V2::new(x1, y1), V2::new(x2, y2), colour);
        }
    }

    /// Draw a rounded rectangle contour
    pub fn round_rect(&mut self, p1: V2, p2: V2, radius: u16, colour: Colour, width: u8) {
        let x1 = min(p1.x, p2.x);
        let x2 = max(p1.x, p2.x);
        let y1 = min(p1.y, p2.y);
        let y2 = max(p1.y, p2.y);

        let hlinestart = (x1 + radius as i16).min(x2);
        let hlineend = (x2 - radius as i16).max(x1);
        self.line(v2(hlinestart, y1), v2(hlineend, y1), colour, width);
        self.line(v2(hlinestart, y2), v2(hlineend, y2), colour, width);

        let vlinestart = (y1 + radius as i16).min(y2);
        let vlineend = (y2 - radius as i16).max(y1);
        self.line(v2(x1, vlinestart), v2(x1, vlineend), colour, width);
        self.line(v2(x2, vlinestart), v2(x2, vlineend), colour, width);

        let radius = (radius as i16, radius as i16);
        self.ellipse_at_center(v2(hlinestart, vlinestart), radius, colour, [true,false,false,false], width);
        self.ellipse_at_center(v2(hlineend, vlinestart), radius, colour, [false,true,false,false], width);
        self.ellipse_at_center(v2(hlinestart, vlineend), radius, colour, [false,false,true,false], width);
        self.ellipse_at_center(v2(hlineend, vlineend), radius, colour, [false,false,false,true], width);
    }

    // Adopted from [Zingl Alois] http://members.chello.at/easyfilter/bresenham.html
    // plot a limited quadratic Bezier segment
    fn quad_bezier_segment(&mut self, p0: V2, p1: V2, p2: V2, colour: Colour, width: u8)
    {
        let (mut x0, x1, mut x2, mut y0, mut y1, mut y2) =
                    (p0.x as i32, p1.x as i32, p2.x as i32, p0.y as i32, p1.y as i32, p2.y as i32);

        let mut seg2_x = x2-x1;
        let mut seg2_y = y2-y1;
        let mut seg1_x = x0-x1;
        let mut seg1_y = y0-y1;
        let mut cur = seg1_x*seg2_y-seg1_y*seg2_x;

        assert!(seg1_x*seg2_x <= 0 && seg1_y*seg2_y <= 0);

        if seg2_x*seg2_x+seg2_y*seg2_y > seg1_x*seg1_x+seg1_y*seg1_y {
            x2 = x0;
            x0 = seg2_x+x1;
            y2 = y0;
            y0 = seg2_y+y1;
            cur = -cur;
        }
        if cur != 0 {
            seg1_x += seg2_x;
            seg2_x = if x0 < x2 {1} else {-1};
            seg1_x *= seg2_x;

            seg1_y += seg2_y;
            seg2_y = if y0 < y2 {1} else {-1};
            seg1_y *= seg2_y;

            let mut xy = 2*seg1_x*seg1_y;
            seg1_x *= seg1_x;
            seg1_y *= seg1_y;

            if cur*seg2_x*seg2_y < 0 {
                seg1_x = -seg1_x; seg1_y = -seg1_y; xy = -xy; cur = -cur;
            }

            let mut dx = 4i32 * seg2_y * cur * (x1-x0) + seg1_x - xy;
            let mut dy = 4i32 * seg2_x * cur * (y0-y1) + seg1_y - xy;

            seg1_x += seg1_x; seg1_y += seg1_y;
            let mut err = dx + dy + xy;

            while dy < 0 && dx > 0 {
                self.thick_pixel(V2::new(x0 as i16, y0  as i16), colour, width);
                if x0 == x2 && y0 == y2 {return;}
                y1 = (2*err < dx) as i32;
                if 2*err > dy {
                    x0 += seg2_x;
                    dx -= xy;
                    dy += seg1_y;
                    err += dy;
                }
                if y1 != 0 {
                    y0 += seg2_y;
                    dy -= xy;
                    dx += seg1_x;
                    err += dx;
                }
            }
        }
        self.line(V2::new(x0 as i16, y0 as i16), V2::new(x2 as i16, y2 as i16), colour, width);
    }


    // Adopted from [Zingl Alois] http://members.chello.at/easyfilter/bresenham.html
    #[allow(non_snake_case)]
    fn _second_segm(&mut self, p0: V2, p1: V2, p2: V2, colour: Colour, width: u8)
    {
        #![allow(clippy::suspicious_operation_groupings)]
        let (x0, x1, x2, y0, y1, y2) = (p0.x as i32, p1.x as i32, p2.x as i32, p0.y as i32, p1.y as i32, p2.y as i32);

        let dy_01 = y0-y1;
        let D = y0-2*y1+y2;

        let xend_D2 = (D-dy_01)*(D-dy_01)*x0  +  2*dy_01*(D-dy_01)*x1  +  dy_01*dy_01*x2;

        let xend = ( (xend_D2 + D*D/2) / (D*D)) as i16;
        let yend = ( ((y0*y2-y1*y1) + D/2) / D ) as i16;

        let xmid_D = (x1-x0)  *  ((y0*y2-y1*y1) - y0*D) / (y1-y0) + x0 * D;
        let xmid = ((xmid_D + D/2) / D) as i16;

        self.quad_bezier_segment(V2::new(x0 as i16,y0 as i16), V2::new(xmid, yend), V2::new(xend,yend), colour, width);

        let xdir_D = (x1-x2)  *  ((y0*y2-y1*y1) - y2*D) / (y1-y2) + x2 * D;
        let xdir = ((xdir_D + D/2) / D) as i16;

        self.quad_bezier_segment(V2::new(xend, yend), V2::new(xdir,yend), V2::new(x2 as i16,y2 as i16), colour, width);
    }


    /// Pixelize and draw a quadratic bezier curve
    #[allow(non_snake_case)]
    pub fn quad_bezier(&mut self, p0: V2, p1: V2, p2: V2, colour: Colour, width: u8)
    {
        #![allow(clippy::suspicious_operation_groupings)]
        // The algorithm is taken from [Zingl Alois] [http://members.chello.at/easyfilter/bresenham.html](http://members.chello.at/easyfilter/bresenham.html)

        let (x0, x1, x2, y0, y1, y2) = (p0.x, p1.x, p2.x, p0.y, p1.y, p2.y);

        if (x0-x1) as i32 * (x2-x1) as i32 > 0 {

            let (x0,y0,x2,y2) =
                if (y0-y1) as i32 * (y2-y1) as i32 > 0 &&
                   ((y0-2*y1+y2) as i32 * (x0-x1) as i32).abs() > ((y0-y1) as i32 * ((x0 - 2 * x1 + x2) as i32)).abs()
                {
                    (x2,y2,x0,y0)
                }
                else {
                    (x0,y0,x2,y2)
                };

            let dx_01 = (x0 - x1) as i32;
            let D = (x0 - 2 * x1 + x2) as i32;

            let yend_D2 = (D-dx_01)*(D-dx_01)*y0 as i32  +  2*(D-dx_01)*dx_01*y1 as i32  +  dx_01*dx_01*y2 as i32;
            let yend = ((yend_D2 + D*D/2) / (D*D)) as i16;
            let xend = (((x0*x2-x1*x1) as i32 + D/2) / D) as i16;

            let ymid_D = (y1-y0) as i32 * ((x0*x2-x1*x1) as i32 - x0 as i32*D) / (x1-x0) as i32 + y0 as i32*D;
            let ymid = ((ymid_D+D/2)/D) as i16;

            self.quad_bezier_segment(V2::new(x0,y0), V2::new(xend,ymid), V2::new(xend,yend), colour, width);

            let ydir_D = (y1-y2)as i32*((x0*x2-x1*x1) as i32 - x2 as i32*D) / (x1-x2)as i32 + y2 as i32*D;
            let ydir = ((ydir_D+D/2)/D) as i16;

            if (yend-ydir) as i32 * (y2-ydir) as i32 > 0 {
                self._second_segm(V2::new(xend, yend), V2::new(xend, ydir), V2::new(x2, y2), colour, width);
            }
            else {
                self.quad_bezier_segment(V2::new(xend, yend), V2::new(xend, ydir), V2::new(x2,y2), colour, width);
            }
        }
        else if (y0-y1) as i32 * (y2-y1) as i32 > 0 {
            self._second_segm(V2::new(x0, y0), V2::new(x1, y1), V2::new(x2, y2), colour, width);
        }
        else {
            self.quad_bezier_segment(V2::new(x0,y0), V2::new(x1,y1), V2::new(x2,y2), colour, width);
        }
    }

    /// quadratic spline between points
    pub fn quad_spline(&mut self, p0: V2, p1: V2, p2: V2, colour: Colour, width: u8)
    {
        let ctrl = (p1*4 - p0 - p2 + v2(1, 1)) / 2;
        self.quad_bezier(v2(p0.x, p0.y), ctrl, v2(p2.x, p2.y), colour, width);
    }

    /// Draw an ellipse contour by center and horizontal/vertical radii
    pub fn ellipse_at_center(&mut self, V2 { x:xm, y:ym }: V2, (a, b): (i16, i16), colour: Colour, quadrants: [bool; 4], width: u8) {
        self.ellipse_at_rect(v2(xm-a, ym-b), v2(xm+a, ym+b), colour, quadrants, width);
    }

    /// Draw an ellipse contour inside a specified rect
    pub fn ellipse_at_rect(&mut self, V2 { x:mut x0, y: mut y0 }: V2, V2 { x:mut x1, y:mut y1 }: V2, colour: Colour, quadrants: [bool; 4], width: u8) {
        let mut set_pixel = |x: i16, y: i16| {self.thick_pixel(v2(x, y), colour, width);};

        let a = (x1-x0).abs();
        let b = (y1-y0).abs();
        let b1 = b & 1;
        let mut dx = 4 * (1-a as i32) * b as i32 * b as i32;
        let mut dy = 4 * (b1 as i32 + 1) * a as i32 * a as i32;
        let mut err = dx+dy+b1 as i32*a as i32*a as i32;
        let mut e2;

        if x0 > x1 { x0 = x1; x1 += a; }
        if y0 > y1 { y0 = y1; }
        y0 += (b+1)/2;
        y1 = y0-b1;
        let a = 8*a as i32*a as i32;
        let b1 = 8*b as i32*b as i32;
        loop {
            if quadrants[3] { set_pixel(x1, y0); }
            if quadrants[2] { set_pixel(x0, y0); }
            if quadrants[0] { set_pixel(x0, y1); }
            if quadrants[1] { set_pixel(x1, y1); }
            e2 = 2*err;
            if e2 <= dy { y0+=1; y1-=1; dy += a; err += dy; }
            if e2 >= dx || 2*err > dy { x0+=1; x1-=1; dx += b1; err += dx;}
            if x0 > x1 {
                break;
            }
        }

        while y0-y1 <= b {
            if quadrants[3] { set_pixel(x0-1, y0); }
            if quadrants[2] { set_pixel(x1+1, y0); }
            if quadrants[0] { set_pixel(x0-1, y1); }
            if quadrants[1] { set_pixel(x1+1, y1); }
            y0 += 1;
            y1 -= 1;
        }
    }

    #[cfg(any(feature="font_data", doc))]
    #[doc(cfg(feature="font_data"))]
    /// Generate and render a symbol
    pub fn symbol(&mut self, ch: char, fontsize: V2, pos: V2, colour: Colour) {
        self.symbol_with_provider(&font_data::TABLES as &[GlyphTable], ch, fontsize, pos, colour);
    }

    #[cfg(any(feature="font_data", doc))]
    #[doc(cfg(feature="font_data"))]
    /// Generate and render a char sequence
    pub fn text(&mut self, s: &str, fontsize: V2, pos: V2, colour: Colour) {
        for (i,c) in s.chars().enumerate() {
            let posx = pos.x + fontsize.x*i as i16;
            if posx < self.size().x && posx + fontsize.x > 0 {
                self.symbol(c, fontsize, V2::new(posx, pos.y), colour);
            }
        }
    }

    /// Generate and render a text with the user-defined steps
    pub fn text_with_provider(&mut self, tables: impl crate::font::GlyphProvider, s: &str, fontsize: V2, pos: V2, colour: Colour) {
        for (i,c) in s.chars().enumerate() {
            let posx = pos.x + fontsize.x*i as i16;
            if posx < self.size().x && posx + fontsize.x > 0 {
                self.symbol_with_provider(tables, c, fontsize, V2::new(posx, pos.y), colour);
            }
        }
    }

    /// Generate and render a symbol with the user-defined steps
    pub fn symbol_with_provider(&mut self, tables: impl crate::font::GlyphProvider, ch: char, fontsize: V2, pos: V2, colour: Colour) {
        let mut sten = crate::helpers::Stencil::new(self, pos, fontsize);
        if let Some(cmds) = tables.get_glyph(ch as u32) {
            crate::font::draw_glyph(cmds, &mut sten, colour, tables);
        }
        else {
            use crate::font::GlyphStep;
            use crate::font::GlyphCoord;
            use crate::font::GlyphConnectionType::Outline;
            crate::font::draw_glyph([
                GlyphStep{coord: GlyphCoord{x: r!([] 0), y: r!([] 24)}, tp: Outline{thick: false, update: true}},
                GlyphStep{coord: GlyphCoord{x: r!([] 12), y: r!([] 24)}, tp: Outline{thick: false, update: true}},
                GlyphStep{coord: GlyphCoord{x: r!([] 12), y: r!([] 0)}, tp: Outline{thick: false, update: true}},
                GlyphStep{coord: GlyphCoord{x: r!([] 0), y: r!([] 0)}, tp: Outline{thick: false, update: true}},
            ].iter().cloned(), &mut sten, colour, tables);
        }
    }
}


#[cfg(test)]
pub(crate) fn canvas_to_string(cvs: &crate::canvas::Canvas<u8>) -> std::string::String {
    use std::fmt::Write;
    let sz = cvs.size();
    let mut s = std::string::String::new();
    for r in 0..sz.y {
        write!(s, "|").unwrap();
        for c in 0..sz.x {
            write!(s, "{} ", cvs.get_pixel(v2(c,r)).unwrap_or(b'?') as char).unwrap();
        }
        writeln!(s, "|").unwrap();
    }
    s
}


#[test]
fn test_ellipse() {
    const SIZE: i16 = 20;

    let mut buffer = [b' '; SIZE as usize * SIZE as usize];
    let mut canvas = crate::canvas::Canvas::<u8>::new(&mut buffer, v2(SIZE, SIZE)).unwrap();
    let c : &mut dyn Drawable<u8> = &mut canvas;

    c.ellipse_at_center(V2 { x: 10, y: 10 }, (5, 8), b'.', [true;4], 1);
    c.ellipse_at_rect(V2 { x: 1, y: 1 }, V2 { x: 15, y: 10 }, b'o', [true;4], 1);

    println!("{}", canvas_to_string(&canvas));

    assert_eq!(canvas_to_string(&canvas).trim(),
"
|                                        |
|          o o o o o o o                 |
|      o o         . . . o o             |
|    o           .       .   o           |
|  o           .           .   o         |
|  o         .               . o         |
|  o         .               . o         |
|  o       .                   o         |
|    o     .                 o .         |
|      o o .             o o   .         |
|          o o o o o o o       .         |
|          .                   .         |
|          .                   .         |
|          .                   .         |
|            .               .           |
|            .               .           |
|              .           .             |
|                .       .               |
|                  . . .                 |
|                                        |
".trim());
}
