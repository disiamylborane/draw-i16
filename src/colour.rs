
/// A RGB565 pixel representation
#[derive(Clone,Copy,PartialEq,Eq)]
#[repr(transparent)]
pub struct Colour565(u16);

impl Colour565 {
    #![allow(missing_docs)]

    /// Create a colour from rgb
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        let rr = (r as u16 & 0b11111000) << 8;
        let gg = (g as u16 & 0b11111100) << 3;
        let bb = (b as u16 & 0b11111000) >> 3;
        Self(rr|gg|bb)
    }

    /// Represent a colour as RGB565
    pub const fn rgb565(self) -> u16 {self.0}

    /// Represent a colour as Red, Green, Blue components
    pub const fn rgb(self) -> (u8,u8,u8) {
        (self.r(), self.g(), self.b())
    }

    /// Get Red component
    pub const fn r(self) -> u8 { (self.0>>8 & 0b11111000) as u8}
    /// Get Green component
    pub const fn g(self) -> u8 { (self.0>>3 & 0b11111100) as u8}
    /// Get Blue component
    pub const fn b(self) -> u8 { (self.0<<3 & 0b11111000) as u8}

    pub const CYAN : Colour565 = Colour565::new(0x00, 0xFF, 0xFF);
    pub const BLACK : Colour565 = Colour565::new(0x00, 0x00, 0x00);
    pub const BLUE : Colour565 = Colour565::new(0x00, 0x00, 0xFF);
    pub const MAGENTA : Colour565 = Colour565::new(0xFF, 0x00, 0xFF);
    pub const GRAY : Colour565 = Colour565::new(0x80, 0x80, 0x80);
    pub const GREEN : Colour565 = Colour565::new(0x00, 0x80, 0x00);
    pub const LIME : Colour565 = Colour565::new(0x00, 0xFF, 0x00);
    pub const MAROON : Colour565 = Colour565::new(0x80, 0x00, 0x00);
    pub const NAVY : Colour565 = Colour565::new(0x00, 0x00, 0x80);
    pub const OLIVE : Colour565 = Colour565::new(0x80, 0x80, 0x00);
    pub const PURPLE : Colour565 = Colour565::new(0x80, 0x00, 0x80);
    pub const RED : Colour565 = Colour565::new(0xFF, 0x00, 0x00);
    pub const SILVER : Colour565 = Colour565::new(0xC0, 0xC0, 0xC0);
    pub const TEAL : Colour565 = Colour565::new(0x00, 0x80, 0x80);
    pub const WHITE : Colour565 = Colour565::new(0xFF, 0xFF, 0xFF);
    pub const YELLOW : Colour565 = Colour565::new(0xFF, 0xFF, 0x00);
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
        assert_eq!(colour, Colour565::new(colour.r(), colour.g(), colour.b()))
    }
}
