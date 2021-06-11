
/// A RGB565 pixel representation
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct Colour(u16);

#[allow(missing_docs)]
impl Colour {
    /// Create a colour from rgb
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        let rr = (r as u16 & 0b11111000) << 8;
        let gg = (g as u16 & 0b11111100) << 3;
        let bb = (b as u16 & 0b11111000) >> 3;
        return Self(rr|gg|bb);
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

    pub const CYAN : Colour = Colour::new(0x00, 0xFF, 0xFF);
    pub const BLACK : Colour = Colour::new(0x00, 0x00, 0x00);
    pub const BLUE : Colour = Colour::new(0x00, 0x00, 0xFF);
    pub const MAGENTA : Colour = Colour::new(0xFF, 0x00, 0xFF);
    pub const GRAY : Colour = Colour::new(0x80, 0x80, 0x80);
    pub const GREEN : Colour = Colour::new(0x00, 0x80, 0x00);
    pub const LIME : Colour = Colour::new(0x00, 0xFF, 0x00);
    pub const MAROON : Colour = Colour::new(0x80, 0x00, 0x00);
    pub const NAVY : Colour = Colour::new(0x00, 0x00, 0x80);
    pub const OLIVE : Colour = Colour::new(0x80, 0x80, 0x00);
    pub const PURPLE : Colour = Colour::new(0x80, 0x00, 0x80);
    pub const RED : Colour = Colour::new(0xFF, 0x00, 0x00);
    pub const SILVER : Colour = Colour::new(0xC0, 0xC0, 0xC0);
    pub const TEAL : Colour = Colour::new(0x00, 0x80, 0x80);
    pub const WHITE : Colour = Colour::new(0xFF, 0xFF, 0xFF);
    pub const YELLOW : Colour = Colour::new(0xFF, 0xFF, 0x00);
}
