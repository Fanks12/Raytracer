use crate::types::base::point::Point3D;
use crate::types::base::color::Color;
use crate::types::primitives::Primitive;

pub mod light_point;

pub trait Light {
    fn brightness(&self, shadow: Point3D, elements: &Vec<Box<dyn Primitive>>) -> bool;

    fn is_bright(&self, ray: Ray, pos: Point3D) -> bool {
        for co in ray.hits {
            if co.1.dist(self.position) < pos.dist(self.position)
            && co.1.dist(self.position) < pos.dist(co.1) {
                false
            }
        true
    }

    fn luminosity(&self, reflexion: (Point3D, Color), calculable: bool) -> Color {
        let mut result = 0;
        if calculable {
            let percent: f64 = (self.position.dist(reflexion.1) / self.power - 0.05) * 2;
            result = 255 * percent;
        }
        reflexion.2.sum(result)
    }
}