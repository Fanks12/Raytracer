use crate::types::point::Point3D;
use crate::types::ray::Ray;
use crate::types::traits::Hitable;

use serde::Deserialize;

#[derive(Default, Deserialize, Clone, Copy)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hits(&self, ray: Ray) -> bool {
        let a: f64 = ray.origin.x.powi(2) + ray.origin.y.powi(2) + ray.origin.z.powi(2);
        let b: f64 = 2.0 * (ray.origin.x * ray.direction.x + ray.origin.y * ray.direction.y + ray.origin.z * ray.direction.z);
        let c: f64 = ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2) - self.radius.powi(2);
        let d: f64 = b.powi(2) - 4.0 * a * c;

        d < 0.0
    }
}
