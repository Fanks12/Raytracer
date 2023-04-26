use crate::types::point::Point3D;
use crate::types::vector::Vector3D;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Rectangle3D {
    pub origin: Point3D,
    pub bottom_side: Vector3D,
    pub left_side: Vector3D,
}

impl Rectangle3D {
    pub fn point_at(&self, x: f64, y: f64) -> Point3D {
        self.origin + self.bottom_side * x + self.left_side * y
    }
}
