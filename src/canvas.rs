//! Picture: the Picture structure


/*
use crate::alloc::boxed::Box;
use crate::memorymap::box_arr_special;
use crate::memorymap::AllocAlgo;
use crate::memorymap::AllocErr;
*/

use super::{Colour, Drawable, V2};

use crate::v2;

/// An error returned by `Canvas::new()` on size mismatch
#[derive(Debug)]
pub struct SizeMismatchError;

/// A reference to Colour array representing a 2D drawing surface
pub struct Canvas<'buf> {
    buffer: &'buf mut [Colour],
    sizex: i16
}

impl<'buf> Canvas<'buf> {
    /// Create a new canvas on the existing buffer
    pub fn new(buffer: &'buf mut [Colour], size: V2) -> Result<Self, SizeMismatchError> { 
        if size.x as usize * size.y as usize == buffer.len() {
            Ok(Self { buffer, sizex: size.x })
        }
        else {
            Err(SizeMismatchError)
        }
    }

    /// Get a pixel by coordinates
    pub fn get_pixel(&self, point: V2) -> Option<Colour> {
        let sz = unsafe{self._size()};
        if 0 <= point.x && point.x <= sz.x &&
           0 <= point.y && point.y <= sz.y 
           {
            Some(self.buffer[point.x as usize + point.y as usize*self.sizex as usize])
        }
        else {
            None
        }
    }
}

impl<'buf> Drawable for Canvas<'buf> {
    unsafe fn _size(&self) -> V2 {v2![self.sizex, self.buffer.len() as i16/self.sizex]}

    unsafe fn _checked_clear(&mut self, colour: Colour) {
        for p in self.buffer.iter_mut() {
            *p = colour;
        }
    }
    unsafe fn _checked_pixel(&mut self, pos: V2, colour: Colour){
        self.buffer[pos.x as usize + pos.y as usize*self.sizex as usize] = colour;
    }
    unsafe fn _checked_hline(&mut self, pos: V2, len: u16, colour: Colour){
        for x in pos.x..(pos.x+len as i16) {
            self.buffer[x as usize + pos.y as usize*self.sizex as usize] = colour;
        }
    }
    unsafe fn _checked_vline(&mut self, pos: V2, len: u16, colour: Colour){
        for y in pos.y..(pos.y+len as i16) {
            self.buffer[pos.x as usize + y as usize*self.sizex as usize] = colour;
        }
    }
    unsafe fn _checked_rect(&mut self, p1: V2, p2: V2, colour: Colour){
        for x in p1.x..(p2.x+1) {
            for y in p1.y..(p2.y+1) {
                self.buffer[x as usize + y as usize*self.sizex as usize] = colour;
            }
        }
    }
}