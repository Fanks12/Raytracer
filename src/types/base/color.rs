use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl Color {
    fn sum(&self, rhs: u8) -> Color {
        Color { r: self.r + rhs, g: self.g + rhs, b: self.b + rhs }
    }
}
