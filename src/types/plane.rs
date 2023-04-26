use crate::types::point::Point3D;
use crate::types::ray::Ray;
use crate::types::traits::Hitable;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
pub struct Plane {
    pub axis: char,
    pub padding :f64,
}

impl Hitable for Plane {
    fn hits(&self, ray: Ray) -> bool {
        let a: f64 = ray.origin.axis_value(self.axis);
        let b: f64 =  ray.origin.axis_value(self.axis) + ray.direction.axis_value(self.axis);
        let diffa: f64 = a - self.padding;
        let diffb: f64 = b - self.padding;

        diffa.powi(2) > diffb.powi(2)
    }
}

impl Plane {
    fn intersect_point(&self, ray: Ray) -> Point3D {
        //let copy: Ray = ray;
        if self.hits(ray) {
            let ratio: f64 = (self.padding - ray.origin.axis_value(self.axis)) / ray.direction.axis_value(self.axis);
            // x = ray.origin.x + ray.direction.x * ratio
            // y = ray.origin.y + ray.direction.y * ratio
            // z = ray.origin.z + ray.direction.z * ratio
            // ou
            ray.origin + ray.direction * ratio
        } else {
            Point3D { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}
