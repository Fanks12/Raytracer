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
    pub fn prop(&self, rhs: f64) -> Color {
        Color { r: self.saturating_mulf(self.r, rhs), g: self.saturating_mulf(self.g, rhs), b: self.saturating_mulf(self.b, rhs) }
    }

    pub fn sum(&self) -> u32 {
        u32::from(self.r) + u32::from(self.g) + u32::from(self.b)
    }

    pub fn saturating_mulf(&self, usign: u8, rhs: f64) -> u8 {
        let mut result: f64 = f64::from(usign) * rhs;
        if result == 0.0 && rhs > 1.0 {
            result = 255.0 * (rhs - 0.8);
        }
        if result > 255.0 {
            255
        } else if result < 0.0 {
            0
        } else {
            result as u8
        }
    }
}
