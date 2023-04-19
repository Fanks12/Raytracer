use crate::types::point::Point3D;
use crate::types::vector::Vector3D;

pub struct Rectangle3D {
    pub origin: Point3D,
    pub bottom_side: Vector3D,
    pub left_side: Vector3D,
}

impl Rectangle3D {
    pub fn point_at(&self, x: f64, y: f64) -> Point3D {
        let mut point = self.origin.clone();

        point += self.bottom_side * x;
        point += self.left_side * y;
        point
    }
}
