use std::ops::{Add, AddAssign, Sub, SubAssign};
use serde::Deserialize;
use crate::types::base::vector::Vector3D;

#[derive(Clone, Copy, Deserialize)]
pub enum AxisEnum {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn to_vec(self) -> Vector3D {
        Vector3D { x: self.x, y: self.y, z: self.z }
    }

    pub fn axis_value(&self, axis: AxisEnum) -> f64 {
        match axis {
            AxisEnum::X => self.x,
            AxisEnum::Y => self.y,
            AxisEnum::Z => self.z,
        }
    }
}

impl Default for Point3D {
    fn default() -> Self {
        Point3D { x: 0.0, y: 0.0, z: 0.0 }
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
