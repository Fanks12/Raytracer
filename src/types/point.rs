use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::types::vector::Vector3D;

#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn to_vec(self) -> Vector3D {
        Vector3D { x: self.x, y: self.y, z: self.z }
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Point3D { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl SubAssign for Point3D {
    fn sub_assign(&mut self, rhs: Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
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

impl AddAssign<Vector3D> for Point3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
