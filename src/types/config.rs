pub mod camera;
pub mod scene;

use serde::Deserialize;
use crate::types::base::vector::Vector3D;

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
