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
            outputs: Vec::new(),
        }
    }

    pub fn forward(&mut self) {
        self.origin.z += 1.0;
    }

    pub fn backward(&mut self) {
        self.origin.z -= 1.0;
    }

    pub fn up(&mut self) {
        self.origin.y += 1.0;
    }

    pub fn down(&mut self) {
        self.origin.y -= 1.0;
    }

    pub fn right(&mut self) {
        self.origin.x += 1.0;
    }

    pub fn left(&mut self) {
        self.origin.x -= 1.0;
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
        // TODO: Camera rotation
        //let norm = self.rotation.normalize();
        let norm = Vector3D { x: 0.0, y: 0.0, z: 1.0 };
        let distance = self.resolution.width as f64 / (self.fov_horizontal / 2.0).tan() as f64;
        let center = Point3D {
            x: self.position.x + norm.x * distance - self.resolution.width as f64 / 2.0,
            y: self.position.y + norm.y * distance - self.resolution.height as f64 / 2.0,
            z: self.position.z + norm.z * distance,
        };

        Camera {
            origin: self.position,
            screen: Rectangle3D {
                origin: center,
                top_side: Vector3D { x: self.resolution.width as f64, ..Default::default() },
                left_side: Vector3D { y: self.resolution.height as f64, ..Default::default()}
            }
        }
    }
}
