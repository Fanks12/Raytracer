use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;
use crate::types::base::color::Color;
use crate::types::primitives::Primitive;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub struct LightPoint {
    pub position: Point3D,
    pub direction: Vector3D,
    pub power: u64,
}

impl Default for LightPoint {
    fn default() -> Self {
        LightPoint { ..Default::default() }
    }
}

impl Light in LightPoint {
    fn brightness(&self, reflexion: (Point3D, Color), elements: &Vec<Box<dyn Primitive>>) -> Color {
        let ray: Ray = Ray {
            origin: reflexion,
            direction: (self.position - reflexion).to_vec(),
            hits: Vec::new(),
        };
        for element in elements {
            element.intersects(ray);
        }
        self.luminosity(reflexion, self.is_bright(ray, reflexion.1))
    }
}
