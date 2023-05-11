use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;
use crate::types::base::color::Color;
use crate::types::primitives::Primitive;
use crate::types::base::ray::Ray;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub struct LightPoint {
    pub position: Point3D,
    pub direction: Vector3D,
    pub power: f64,
}

impl Default for LightPoint {
    fn default() -> Self {
        LightPoint { ..Default::default() }
    }
}

impl LightPoint {
    pub fn brightness(&self, reflexion: (Point3D, Color), elements: &Vec<Box<dyn Primitive>>) -> Color {
        let mut ray: Ray = Ray {
            origin: reflexion.0,
            direction: (self.position - reflexion.0).to_vec(),
            hits: Vec::new(),
            outputs: Vec::new(),
        };
        for element in elements {
            element.intersects(&mut ray);
        }
        self.luminosity(reflexion, self.is_bright(ray, reflexion.0))
    }

    fn is_bright(&self, ray: Ray, pos: Point3D) -> bool {
        if ray.closest().0.dist(self.position) < pos.dist(self.position)
        && ray.closest().0.dist(self.position) < pos.dist(ray.closest().0) {
            return false;
        }
        return true;
    }

    fn luminosity(&self, reflexion: (Point3D, Color), calculable: bool) -> Color {
        if calculable {
            let mut percent: f64 = 1.0 - self.position.dist(reflexion.0) / self.power - 0.5;
            if self.power == 0.0 || self.position.dist(reflexion.0) > self.power {
                percent = -0.8;
            }
            let result: f64 = 255.0 * percent;
            if result < 0.0 {
                return reflexion.1.dif(result.abs() as u8);
            } else {
                return reflexion.1.addi(result as u8);
            }
        }
        Color { r: 0, g: 0, b: 0 }
    }
}
