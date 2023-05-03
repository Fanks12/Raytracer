use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Default for Color {
    fn default() -> Self {
        Color { red: 0, green: 0, blue: 0 }
    }
}
