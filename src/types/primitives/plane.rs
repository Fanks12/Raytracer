use serde::Deserialize;
use crate::types::base::point::AxisEnum;
use crate::types::primitives::Primitive;
use crate::types::base::color::Color;
use crate::types::base::ray::Ray;

#[derive(Deserialize, Copy, Clone)]
pub struct Plane {
    pub axis: AxisEnum,
    pub padding :f64,
    pub color: Color,
}

impl Primitive for Plane {
    fn intersects(&self, ray: &mut Ray) {
        let a: f64 = ray.origin.axis_value(self.axis);
        let b: f64 =  ray.origin.axis_value(self.axis) + ray.direction.axis_value(self.axis);
        let diffa: f64 = a + self.padding;
        let diffb: f64 = b + self.padding;
        if diffa.abs() > diffb.abs() {
            let ratio: f64 = (-self.padding - ray.origin.axis_value(self.axis)) / ray.direction.axis_value(self.axis);
            ray.hits.push((ray.origin + ray.direction * ratio, self.color));
        }
    }

    fn get_color(&self) -> &Color {
        &self.color
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane { axis: AxisEnum::Z, padding: 0.0, color: Default::default() }
    }
}