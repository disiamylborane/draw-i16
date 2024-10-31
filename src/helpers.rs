//! helpers: the reference-like wrappers for drawables manipulation

use super::{Drawable, V2, v2};

/// A 2D-reference-like type for `Drawable`, which
/// points to a window of some Drawable. Using the stencil,
/// one can't draw outside its size
///
/// ```text
/// Original -> Apply Stencil  ->  Draw on  ->  Result on original
/// drawable    origin: (1,0)      Stencil      is moved
///             size: (2,2)
///
/// |o....|         |..|            |a.|          |oa...|
/// |.o...|         |o.|            |ob|          |.ob..|
/// ```
///
/// # Example
/// ```
/// # use draw_i16::*; use core::str;
/// let mut buffer = [b'.'; 2*5];
/// let mut canvas = Canvas::new(&mut buffer, v2(5,2)).unwrap();
/// canvas.pixel(v2(0, 0), b'o');
/// canvas.pixel(v2(1, 1), b'o');
/// let mut stencil = Stencil::new(&mut canvas, v2(1,0), v2(2,2) );
/// stencil.pixel(v2(0, 0), b'a');
/// stencil.pixel(v2(1, 1), b'b');
/// assert_eq!(str::from_utf8(&buffer).unwrap(), "oa....ob..");
/// ```
pub struct Stencil<'a, Colour:Copy> {
    child: &'a mut dyn Drawable<Colour>,
    origin: V2,
    size: V2
}

impl<'a, Colour:Copy> Stencil<'a, Colour> {
    /// Create a new `Stencil`
    pub fn new(child: &'a mut dyn Drawable<Colour>, origin: V2, size: V2)->Self {
        Self{child,origin,size}
    }
}

impl<Colour:Copy> Drawable<Colour> for Stencil<'_, Colour> {
    fn _size(&self) -> V2 {self.size}

    fn _clear(&mut self, colour: Colour){
       self.child.rect_fill(self.origin, self.origin+self.size, colour);
    }
    unsafe fn _pixel(&mut self, pos: V2, colour: Colour){
        self.child.pixel(self.origin + pos, colour);
    }
    unsafe fn _hline(&mut self, pos: V2, len: u16, colour: Colour){
        self.child.horz_line(self.origin + pos, len, colour);
    }
    unsafe fn _vline(&mut self, pos: V2, len: u16, colour: Colour){
        self.child.vert_line(self.origin + pos, len, colour);
    }
    unsafe fn _rect(&mut self, p1: V2, p2: V2, colour: Colour){
        self.child.rect_fill(self.origin + p1, self.origin + p2, colour);
    }
}

/// A helper for `Rotator` to specify the rotation behaviour
#[derive(Clone,Copy)]
pub enum RotationType{
    /// The Rotator has no effect
    NoRotation,
    /// Paintings are rotated by 90 degrees clockwise
    CW,
    /// Paintings are rotated by 180 degrees
    Flip,
    /// Paintings are rotated by 90 degrees counter-clockwise
    CCW
}

/// Rotator the drawing to its child to either 90, 180 or 270 degrees.
///
/// ```text
/// Original -> Apply Rotator     ->  Draw on  ->  Result on original
/// drawable    RotationType::CW      Rotator      is rotated clockwise
///
/// |o....|          |..|               |a.|    ↻     |o.cba|
/// |.o...|          |..|               |b.|  ---->   |.o...|
///                  |..|               |c.|
///                  |.o|               |.o|
///                  |o.|               |o.|
/// original        rotator            rotator        original
/// surface         surface            surface        surface
/// ```
///
/// # Example
/// ```
/// # use draw_i16::*; use core::str;
/// let mut buffer = [b'.'; 2*5];
/// let mut canvas = Canvas::new(&mut buffer, v2(5,2)).unwrap();
/// canvas.pixel(v2(0, 0), b'o');
/// canvas.pixel(v2(1, 1), b'o');
/// let mut rotated = Rotator::new(&mut canvas, RotationType::CW);
/// rotated.pixel(v2(0, 0), b'a');
/// rotated.pixel(v2(0, 1), b'b');
/// rotated.pixel(v2(0, 2), b'c');
/// assert_eq!(str::from_utf8(&buffer).unwrap(), "o.cba.o...");
/// ```
pub struct Rotator<'a, Colour:Copy> {
    child: &'a mut dyn Drawable<Colour>,
    rot: RotationType,
}
impl<'a, Colour:Copy> Rotator<'a, Colour> {
    /// Create a new `Rotator` to draw rotated with `rot` type onto `child` drawable
    pub fn new(child: &'a mut dyn Drawable<Colour>, rot: RotationType)->Self {
        Self{child, rot}
    }

    fn coord_to_child(&self, r: V2) -> V2 {
        let sz = self.child._size();

        match self.rot {
            RotationType::NoRotation => r,
            RotationType::CW => v2(sz.x-1-r.y, r.x),
            RotationType::Flip => v2(sz.x-1-r.x, sz.y-1-r.y),
            RotationType::CCW => v2(r.y, sz.y-1-r.x),
        }
    }
}
impl<Colour:Copy> Drawable<Colour> for Rotator<'_, Colour> {
    fn _size(&self) -> V2 {
        match self.rot {
            RotationType::NoRotation|RotationType::Flip => {
                self.child._size()
            }
            RotationType::CW|RotationType::CCW => {
                self.child._size().swap()
            }
        }
    }

    fn _clear(&mut self, colour: Colour) {
       self.child._clear(colour);
    }
    unsafe fn _pixel(&mut self, pos: V2, colour: Colour) {
        self.child._pixel(self.coord_to_child(pos), colour);
    }

    unsafe fn _hline(&mut self, pos: V2, len: u16, colour: Colour){

        match self.rot {
            RotationType::NoRotation => {
                self.child._hline(pos, len, colour);
            }
            RotationType::CW => {
                self.child._vline(self.coord_to_child(pos + v2(0, 0)), len, colour);
            }
            RotationType::Flip => {
                self.child._hline(self.coord_to_child(pos + v2(len as i16-1, 0)), len, colour);
            }
            RotationType::CCW => {
                self.child._vline(self.coord_to_child(pos + v2(len as i16-1, 0)), len, colour);
            }
        }
    }
    unsafe fn _vline(&mut self, pos: V2, len: u16, colour: Colour){
        match self.rot {
            RotationType::NoRotation => {
                self.child._vline(pos, len, colour);
            }
            RotationType::CW => {
                self.child._hline(self.coord_to_child(pos + v2(0, len as i16-1)), len, colour);
            }
            RotationType::Flip => {
                self.child._vline(self.coord_to_child(pos + v2(0, len as i16-1) ), len, colour);
            }
            RotationType::CCW => {
                self.child._hline(self.coord_to_child(pos), len, colour);
            }
        }
    }
    unsafe fn _rect(&mut self, p1: V2, p2: V2, colour: Colour){
        let pp1 = self.coord_to_child(p1);
        let pp2 = self.coord_to_child(p2);
        self.child.rect_fill(
            V2{x:pp1.x, y:pp2.y},
            V2{x:pp2.x, y:pp1.y},
            colour);
    }
}

#[test]
fn test_rotator() {
    use crate::*;
    let mut buffer = [b'.'; 2*3];
    let mut canvas = Canvas::new(&mut buffer, v2(2,3)).unwrap();
    Rotator::new(&mut canvas, RotationType::CW).pixel(v2(1, 0), b'a');
    Rotator::new(&mut canvas, RotationType::Flip).pixel(v2(1, 0), b'b');
    Rotator::new(&mut canvas, RotationType::CCW).pixel(v2(1, 0), b'c');
    println!("{}", drawable::canvas_to_string(&canvas));
    assert_eq!(core::str::from_utf8(&buffer).unwrap(), "..cab.");
}
