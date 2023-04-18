//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-florian.lauch
// File description:
// vector3d
//

use std::ops;

#[derive(Default, Debug)]
pub struct Vector3D {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, other : Self) -> Self {
        Self {x : self.x + other.x, y : self.y + other.y, z : self.z + other.z}
    }
}

impl Vector3D {
    pub fn new(x : f64, y : f64, z : f64) -> Self {
        Self {x, y, z}
    }
}
