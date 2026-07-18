#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn from_u32(c: u32) -> Color {
        let a = (c >> 24) as u8;
        let r = (c >> 16) as u8;
        let g = (c >> 08) as u8;
        let b = (c >> 00) as u8;
        Color { a, r, g, b }
    }
}
