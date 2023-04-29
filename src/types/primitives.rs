use crate::types::base::point::Point3D;
use crate::types::ray::Ray;

pub mod plane;
pub mod sphere;

pub trait Primitive {
    fn hits(&self, ray: Ray) -> bool;
    fn intersect_point(&self, ray: Ray) -> Vec<Point3D>;
}
