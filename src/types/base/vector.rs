use std::ops::{Add, AddAssign, Mul, MulAssign};
use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn dot_product(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross_product(&self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x}
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let length = self.length();
        if length <= 0.0 {
            return Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        }
        Vector3D { x: self.x / length, y: self.y / length, z: self.z / length }
    }

    pub fn rotate(&mut self, axis: Vector3D, angle: f64) {
            let norm = self.normalize();
            let cos_angle = angle.cos();
            let sin_angle = angle.sin();
            let dot_product = norm.dot_product(axis);
            let cross_product = norm.cross_product(axis);
            let v_rotated = Vector3D {
                x: cos_angle * norm.x + (1.0 - cos_angle) * dot_product * axis.x + sin_angle * cross_product.x,
                y: cos_angle * norm.y + (1.0 - cos_angle) * dot_product * axis.y + sin_angle * cross_product.y,
                z: cos_angle * norm.z + (1.0 - cos_angle) * dot_product * axis.z + sin_angle * cross_product.z };
            self.x = v_rotated.x;
            self.y = v_rotated.y;
            self.z = v_rotated.z;
        }
}

impl Default for Vector3D {
    fn default() -> Self {
        Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3D { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
