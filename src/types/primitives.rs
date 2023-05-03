use crate::types::base::point::Point3D;
use crate::types::base::ray::Ray;

pub mod plane;
pub mod sphere;

pub trait Primitive {
    fn intersects(&self, ray: &mut Ray);
}