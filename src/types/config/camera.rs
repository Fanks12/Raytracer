use serde::Deserialize;
use crate::types::base::point::Point3D;
use crate::types::config::Resolution;
use crate::types::base::ray::Ray;
use crate::types::base::rectangle::Rectangle3D;
use crate::types::base::vector::Vector3D;

#[derive(Default)]
pub struct Camera {
    pub origin: Point3D,
    pub screen: Rectangle3D,
}

impl Camera {
    pub fn ray(&self, x: f64, y: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.screen.point_at(x, y) - self.origin).to_vec(),
            hits: Vec::new(),
        }
    }
}

#[derive(Default, Deserialize)]
pub struct CamConfig {
    pub position: Point3D,
    pub rotation: Point3D,
    pub fov_horizontal: f32,
    pub fov_vertical: f32,
    pub resolution: Resolution,
}

impl CamConfig {
    pub fn convert(&self) -> Camera {
        let norm = self.rotation.normalize();
        let distance = self.position.x / (self.fov_horizontal / 2.0).tan();
        let center = Vector3D {
            x: self.position.x + norm.x * distance - self.position.x,
            y: self.position.y + norm.y * distance - self.position.y,
            z: self.position.z + norm.z * distance - self.position.z,
        };
    }
}