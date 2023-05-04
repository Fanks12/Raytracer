use serde::Deserialize;
use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;

#[derive(Default, Deserialize)]
pub struct Rectangle3D {
    pub origin: Point3D,
    pub top_side: Vector3D,
    pub left_side: Vector3D,
}

impl Rectangle3D {
    pub fn point_at(&self, x: f64, y: f64) -> Point3D {
        self.origin + self.bottom_side * x + self.left_side * y
    }
}
