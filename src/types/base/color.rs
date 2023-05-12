use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
