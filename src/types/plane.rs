use crate::types::point::Point3D;
use crate::types::ray::Ray;
use crate::types::traits::Hitable;

pub struct Plane {
    pub axis: f32,
    pub padding :f64,
    pub color: f64,
}

impl Hitable for Plane {
    fn hits(&self, ray: Ray) -> bool {
        let a: f64 = ray.origin[axis]
        let b: f64 =  ray.origin[axis] + ray.vector[axis]
        let diffa: f64 = a - padding
        let diffb: f64 = b - padding

        diffa.powi(2) > diffb.powi(2)
    }
}
