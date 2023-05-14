use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;
use crate::types::base::color::Color;
use crate::types::primitives::Primitive;
use crate::types::base::ray::Ray;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Default)]
pub struct LightPoint {
    pub position: Point3D,
    pub direction: Vector3D,
    pub power: f64,
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
        self.luminosity(reflexion)
    }

    fn luminosity(&self, reflexion: (Point3D, Color)) -> Color {
        let angular: Vector3D = (reflexion.0 - self.position).to_vec();
        let distance: f64 = self.position.dist(reflexion.0);
        let mut percent: f64 = 0.0;
        if distance < self.power * 0.1 {
            percent = 1.1 - distance / self.power - self.direction.vector_to_angle(angular) * 180.0 / std::f64::consts::PI / 50.0;
        } else if distance < self.power {
            percent = (1.0 - distance / self.power) - self.direction.vector_to_angle(angular) * 180.0 / std::f64::consts::PI / 35.0;
        }
        reflexion.1.prop(percent)
    }
}
