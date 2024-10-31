//! Canvas: a drawable from a slice of pixels

use super::{Drawable, V2, v2};

/// An error returned by `Canvas::new()` on size mismatch
#[derive(Debug)]
pub struct SizeMismatchError;

/// A reference to Colour slice representing a 2D drawing surface
///
/// # Example
/// ```
/// # use draw_i16::*; use core::str;
/// let mut buffer = [b'.'; 3*5];
/// let mut canvas = Canvas::<u8>::new(&mut buffer, v2(5,3)).unwrap();
/// canvas.rect(v2(0, 0), v2(3, 2), b'o');
/// assert_eq!(str::from_utf8(&buffer).unwrap(),
///                "oooo.".to_string() +
///                "o..o." +
///                "oooo.");
/// ```
pub struct Canvas<'buf, Colour:Copy> {
    buffer: &'buf mut [Colour],
    sizex: i16
}

impl<'buf, Colour:Copy> Canvas<'buf, Colour> {
    /// Create a new canvas on the existing buffer
    /// 
    /// The buffer size must be exactly size.x * size.y
    /// 
    /// # Errors
    /// `SizeMismatchError` on invalid canvas size
    pub fn new(buffer: &'buf mut [Colour], size: V2) -> Result<Self, SizeMismatchError> {
        if size.x as usize * size.y as usize == buffer.len() {
            Ok(Self { buffer, sizex: size.x })
        }
        else {
            Err(SizeMismatchError)
        }
    }

    /// Get a pixel by coordinates
    #[must_use]
    pub fn get_pixel(&self, point: V2) -> Option<Colour> {
        let sz = self._size();
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

impl<Colour:Copy> Drawable<Colour> for Canvas<'_, Colour> {
    fn _size(&self) -> V2 {v2(self.sizex, (self.buffer.len()/self.sizex as usize) as i16)}

    fn _clear(&mut self, colour: Colour) {
        for p in self.buffer.iter_mut() {
            *p = colour;
        }
    }
    unsafe fn _pixel(&mut self, pos: V2, colour: Colour){
        self.buffer[pos.x as usize + pos.y as usize*self.sizex as usize] = colour;
    }
    unsafe fn _hline(&mut self, pos: V2, len: u16, colour: Colour){
        for x in pos.x..(pos.x+len as i16) {
            self.buffer[x as usize + pos.y as usize*self.sizex as usize] = colour;
        }
    }
    unsafe fn _vline(&mut self, pos: V2, len: u16, colour: Colour){
        for y in pos.y..(pos.y+len as i16) {
            self.buffer[pos.x as usize + y as usize*self.sizex as usize] = colour;
        }
    }
    unsafe fn _rect(&mut self, p1: V2, p2: V2, colour: Colour){
        for x in p1.x..=p2.x {
            for y in p1.y..=p2.y {
                self.buffer[x as usize + y as usize*self.sizex as usize] = colour;
            }
        }
    }
}
