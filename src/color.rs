pub struct Color{
    r: u8,
    g: u8,
    b: u8
}

impl Color{
    pub const fn new(r:u8, g:u8, b:u8) -> Self{
        Self{
            r: r,
            g: g,
            b: b
        }
    }

    pub fn to_u32(&self) -> u32{
        ((self.r as u32) << 16) |
        ((self.g as u32) << 8)  |
        (self.b as u32)
    }
}