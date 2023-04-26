use crate::types::point::Point3D;
use crate::types::ray::Ray;
use crate::types::traits::Hitable;

pub struct Plane {
    pub axis: char,
    pub padding :f64,
    pub color: f64,
}

impl Hitable for Plane {
    fn hits(&self, ray: Ray) -> bool {
        let a: f64 = ray.origin.axisValue(axis)
        let b: f64 =  ray.origin.axisValue(axis) + ray.vector.axisValue(axis)
        let diffa: f64 = a - padding
        let diffb: f64 = b - padding

        diffa.powi(2) > diffb.powi(2)
    }

    fn intersectPoint(&self, ray: Ray) -> Point3D {
        if (self.hits(ray)) {
            // ratio = (padding - a) / ray.vector.axisValue(axis)
            // x = origin.x + vector.x * ratio
            // y = origin.y + vector.y * ratio
            // z = origin.z + vector.z * ratio
            // ou
            origin + vector * ratio
        } else {
            Point3D { x: 0.0, y: 0.0, z: 0.0 }
        }

    }
}
