use serde::Deserialize;
use crate::types::base::point::{AxisEnum, Point3D};
use crate::types::primitives::Primitive;
use crate::types::ray::Ray;

#[derive(Deserialize, Copy, Clone)]
pub struct Plane {
    pub axis: AxisEnum,
    pub padding :f64,
}

impl Primitive for Plane {
    fn hits(&self, ray: Ray) -> bool {
        let a: f64 = ray.origin.axis_value(self.axis);
        let b: f64 =  ray.origin.axis_value(self.axis) + ray.direction.axis_value(self.axis);
        let diffa: f64 = a - self.padding;
        let diffb: f64 = b - self.padding;

        diffa.abs() >= diffb.abs()
    }
    fn intersect_point(&self, ray: Ray) -> Vec<Point3D> {
        let mut tab = Vec::new();
        if self.hits(ray) {
            let ratio: f64 = (self.padding - ray.origin.axis_value(self.axis)) / ray.direction.axis_value(self.axis);
            tab.push(ray.origin + ray.direction * ratio);
        }
        tab
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane { axis: AxisEnum::Z, padding: 0.0 }
    }
}