//! 2D vector of i16
//!
//! The structure represents a coordinates of a pixel
//!
//! # Example
//! ```
//! # use draw_i16::*;
//! let point1 = v2(5,10);
//! let point2 = v2(10,15);
//! assert_eq!(point1 + point2, v2(15,25));
//! ```

/// 2D vector with i16 coordinates
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct V2 {
    /// X coordinate of a vector
    pub x: i16,
    /// Y coordinate of a vector
    pub y: i16
}
impl V2 {
    /// Create a new vector (consider using v2! macro instead)
    #[must_use] pub const fn new(x: i16, y: i16) -> Self {Self{x,y}}

    /// Convert (x,y) to (y,x)
    #[must_use] pub const fn swap(self) -> Self {Self{x: self.y, y: self.x}}
}
impl core::ops::Add for V2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {Self{x: self.x + rhs.x, y: self.y + rhs.y}}
}
impl core::ops::Sub for V2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {Self{x: self.x - rhs.x, y: self.y - rhs.y}}
}
impl core::ops::Mul<i8> for V2 {
    type Output = Self;
    fn mul(self, rhs: i8) -> Self::Output {Self{x: self.x * rhs as i16, y: self.y * rhs as i16}}
}
impl core::ops::Div<i16> for V2 {
    type Output = Self;
    fn div(self, rhs: i16) -> Self::Output {Self{x: self.x / rhs, y: self.y / rhs}}
}

/// Create a new [`V2`]
pub const fn v2(x: i16, y: i16) -> V2 {
    V2{x, y}
}
