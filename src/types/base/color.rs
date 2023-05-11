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
        Color { r: self.r.saturating_add(rhs), g: self.g.saturating_add(rhs), b: self.b.saturating_add(rhs) }
    }
    fn dif(&self, rhs: u8) -> Color {
        Color { r: self.r.saturating_sub(rhs), g: self.g.saturating_sub(rhs), b: self.b.saturating_sub(rhs) }
    }
}
