use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn dot_product(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn axisValue(&self, c: char) -> f64 {
        if (c == 'x')
            self.x
        else if (c == 'y')
            self.y
        else if (c == 'z')
            self.z
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
