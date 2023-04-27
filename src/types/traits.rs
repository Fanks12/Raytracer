use crate::types::ray::Ray;
use crate::types::point::Point3D;

pub trait Hitable {
    fn hits(&self, ray: Ray) -> bool;
    fn intersect_point(&self, ray: Ray) -> Vec<Point3D>;
}
