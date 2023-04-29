pub mod camera;
pub mod scene;

use serde::Deserialize;
use crate::types::base::vector::Vector3D;

#[derive(Default, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Default, Deserialize)]
pub struct Light {
    pub pos : Vector3D,
    pub angle : f32,
}

#[derive(Default, Deserialize)]
pub struct Resolution {
    pub height: usize,
    pub width: usize,
}
