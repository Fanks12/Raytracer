pub mod camera;
pub mod scene;

use serde::Deserialize;
use crate::types::base::vector::Vector3D;

#[derive(Default, Deserialize)]
pub struct Resolution {
    pub height: usize,
    pub width: usize,
}
