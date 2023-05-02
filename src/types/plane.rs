use crate::types::traits::Primitives;
use crate::types::color::Color;
use crate::types::ray::Ray;

#[derive(Default, Copy, Clone)]
pub struct Plane {
    pub axis: char,
    pub padding :f64,
    pub color: Color,
}

impl Primitives for Plane {
    fn intersects(&self, ray: &mut Ray) {
        let a: f64 = ray.origin.axis_value(self.axis);
        let b: f64 =  ray.origin.axis_value(self.axis) + ray.direction.axis_value(self.axis);
        let diffa: f64 = a - self.padding;
        let diffb: f64 = b - self.padding;
        if diffa.abs() >= diffb.abs() {
            let ratio: f64 = (self.padding - ray.origin.axis_value(self.axis)) / ray.direction.axis_value(self.axis);
            ray.hits.push((ray.origin + ray.direction * ratio, self.color));
        }
    }
}
