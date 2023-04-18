use std::ops::Add;
use crate::types::vector::Vector3D;

pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
