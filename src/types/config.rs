pub mod camera;
pub mod scene;

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}
