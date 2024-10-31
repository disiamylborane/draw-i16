
/// A RGB565 pixel representation
#[derive(Clone,Copy,PartialEq,Eq)]
#[repr(transparent)]
pub struct Colour565(u16);

impl Colour565 {
    #![allow(missing_docs)]

    /// Create a colour from rgb
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        let rr = (r as u16 & 0b1111_1000) << 8;
        let gg = (g as u16 & 0b1111_1100) << 3;
        let bb = (b as u16 & 0b1111_1000) >> 3;
        Self(rr|gg|bb)
    }

    /// Represent a colour as RGB565
    #[must_use]
    pub const fn rgb565(self) -> u16 {self.0}

    /// Represent a colour as Red, Green, Blue components
    #[must_use]
    pub const fn rgb(self) -> (u8,u8,u8) {
        (self.r(), self.g(), self.b())
    }

    /// Get Red component
    #[must_use]
    pub const fn r(self) -> u8 { (self.0>>8 & 0b11111000) as u8}
    /// Get Green component
    #[must_use]
    pub const fn g(self) -> u8 { (self.0>>3 & 0b11111100) as u8}
    /// Get Blue component
    #[must_use]
    pub const fn b(self) -> u8 { (self.0<<3 & 0b11111000) as u8}

    pub const CYAN : Self = Self::new(0x00, 0xFF, 0xFF);
    pub const BLACK : Self = Self::new(0x00, 0x00, 0x00);
    pub const BLUE : Self = Self::new(0x00, 0x00, 0xFF);
    pub const MAGENTA : Self = Self::new(0xFF, 0x00, 0xFF);
    pub const GRAY : Self = Self::new(0x80, 0x80, 0x80);
    pub const GREEN : Self = Self::new(0x00, 0x80, 0x00);
    pub const LIME : Self = Self::new(0x00, 0xFF, 0x00);
    pub const MAROON : Self = Self::new(0x80, 0x00, 0x00);
    pub const NAVY : Self = Self::new(0x00, 0x00, 0x80);
    pub const OLIVE : Self = Self::new(0x80, 0x80, 0x00);
    pub const PURPLE : Self = Self::new(0x80, 0x00, 0x80);
    pub const RED : Self = Self::new(0xFF, 0x00, 0x00);
    pub const SILVER : Self = Self::new(0xC0, 0xC0, 0xC0);
    pub const TEAL : Self = Self::new(0x00, 0x80, 0x80);
    pub const WHITE : Self = Self::new(0xFF, 0xFF, 0xFF);
    pub const YELLOW : Self = Self::new(0xFF, 0xFF, 0x00);
}

impl core::fmt::Debug for Colour565 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:x}{:x}{:x}", self.r(), self.g(), self.b())
    }
}

#[test]
fn test_recreate() {
    let iter = (0..0xFF).step_by(5)
        .flat_map(|i| (7..0xFF).step_by(5).map(move |r| (i,r)))
        .flat_map(|(i,r)| (19..0xFF).step_by(5).map(move |j| (j,i,r)));

    for (r,g,b) in iter {
        let colour = Colour565::new(r,g,b);
        assert_eq!(colour, Colour565::new(colour.r(), colour.g(), colour.b()));
    }
}
