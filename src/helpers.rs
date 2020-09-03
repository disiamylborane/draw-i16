//! helpers: the reference-like wrappers for drawables manipulation

use super::{Colour, Drawable, V2};

/// Stencil is a 2D-reference-like type for `Drawable`.
/// Stencil points to a window of some Drawable. Using the stencil,
/// one can't draw outside its size
/// 
/// # Example
/// ```
/// let mut stencil = Stencil::new(&lcd, v2![20,20], v2![10,20]);  // A vertical 10x20 window on lcd
/// stencil.pixel(v2![5,10], Colour::YELLOW);  // A yellow pixel
/// stencil.pixel(v2![20,10], Colour::YELLOW);  // No effect
/// ```
pub struct Stencil<'a> {
    child: &'a mut dyn Drawable,
    origin: V2,
    size: V2
}

impl<'a> Stencil<'a> {
    /// Create a new `Stencil`
    pub fn new(child: &'a mut dyn Drawable, origin: V2, size: V2)->Self {
        Self{child,origin,size}
    }
}

impl Drawable for Stencil<'_> {
    unsafe fn _size(&self) -> V2 {self.size}

    unsafe fn _checked_clear(&mut self, colour: Colour){
       self.child.rect_fill(self.origin, self.origin+self.size, colour);
    }
    unsafe fn _checked_pixel(&mut self, pos: V2, colour: Colour){
        self.child.pixel(self.origin + pos, colour);
    }
    unsafe fn _checked_hline(&mut self, pos: V2, len: u16, colour: Colour){
        self.child.horz_line(self.origin + pos, len, colour);
    }
    unsafe fn _checked_vline(&mut self, pos: V2, len: u16, colour: Colour){
        self.child.vert_line(self.origin + pos, len, colour);
    }
    unsafe fn _checked_rect(&mut self, p1: V2, p2: V2, colour: Colour){
        self.child.rect_fill(self.origin + p1, self.origin + p2, colour);
    }
}

/// A helper for `Rotator` to specify the rotation behaviour
#[derive(Clone,Copy)]
pub enum RotationType{
    /// The Rotator has no effect
    NoRotation,
    /// TODO
    CW,
    /// TODO
    Flip,
    /// TODO
    CCW
}

/// Rotator turns it's child to either 90, 180 or 270 degrees.
pub struct Rotator<'a> {
    child: &'a mut dyn Drawable,
    rot: RotationType,
}
impl<'a> Rotator<'a> {
    /// Create a new `Rotator`
    pub fn new(child: &'a mut dyn Drawable, rot: RotationType)->Self {
        Self{child, rot}
    }

    unsafe fn coord_to_child(&self, r: V2) -> V2 {
        let sz = self.child._size();

        match self.rot {
            RotationType::NoRotation => {
                r
            }
            RotationType::CW => {
                V2{x: sz.x-r.y, y: r.x}
            }
            RotationType::Flip => {
                V2{x: sz.x-r.x, y: sz.y-r.y}
            }
            RotationType::CCW => {
                V2{x: r.y, y: sz.y-r.x}
            }
        }
    }
}
impl Drawable for Rotator<'_> {
    unsafe fn _size(&self) -> V2 {
        match self.rot {
            RotationType::NoRotation => {
                self.child._size()
            }
            RotationType::CW => {
                self.child._size().swap()
            }
            RotationType::Flip => {
                self.child._size()
            }
            RotationType::CCW => {
                self.child._size().swap()
            }
        }
    }

    unsafe fn _checked_clear(&mut self, colour: Colour) {
       self.child._checked_clear(colour)
    }
    unsafe fn _checked_pixel(&mut self, pos: V2, colour: Colour){
        self.child._checked_pixel(self.coord_to_child(pos), colour)
    }
    
    unsafe fn _checked_hline(&mut self, pos: V2, len: u16, colour: Colour){

        match self.rot {
            RotationType::NoRotation => {
                self.child._checked_hline(pos, len, colour);
            }
            RotationType::CW => {
                self.child._checked_vline(self.coord_to_child(pos+V2{x:0, y:0}), len, colour);
            }
            RotationType::Flip => {
                self.child._checked_hline(self.coord_to_child(pos + v2!(len as i16-1, 0)), len, colour);
            }
            RotationType::CCW => {
                self.child._checked_vline(self.coord_to_child( pos+v2!(len as i16-1, 0) ), len, colour);
            }
        }
    }
    unsafe fn _checked_vline(&mut self, pos: V2, len: u16, colour: Colour){
        match self.rot {
            RotationType::NoRotation => {
                self.child._checked_vline(pos, len, colour);
            }
            RotationType::CW => {
                self.child._checked_hline(self.coord_to_child(pos + v2!(0, len as i16-1)), len, colour);
            }
            RotationType::Flip => {
                self.child._checked_vline(self.coord_to_child(pos + v2!(0, len as i16-1) ), len, colour);
            }
            RotationType::CCW => {
                self.child._checked_hline(self.coord_to_child(pos), len, colour);
            }
        }
    }
    unsafe fn _checked_rect(&mut self, p1: V2, p2: V2, colour: Colour){
        let pp1 = self.coord_to_child(p1);
        let pp2 = self.coord_to_child(p2);
        self.child.rect_fill(
            V2{x:pp1.x, y:pp2.y},
            V2{x:pp2.x, y:pp1.y},
            colour);
    }
}
