use crate::types::ray::Ray;

pub mod point;
pub mod ray;
pub mod sphere;
pub mod vector;

pub trait HasLength {
    fn length(&self) -> f64;
}

pub trait Hitable {
    fn hits(&self, ray: Ray) -> bool;
}
