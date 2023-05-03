use serde::Deserialize;
use crate::types::base::point::Point3D;
use crate::types::config::Resolution;
use crate::types::base::ray::Ray;
use crate::types::rectangle::Rectangle3D;

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
        }
    }
}

#[derive(Default, Deserialize)]
pub struct CamConfig {
    pub position: Point3D,
    pub rotation: Point3D,
    pub field_of_view_vert: f32,
    pub field_of_view_hori: f32,
    pub resolution: Resolution,
}

impl CamConfig {
    pub fn convert(&self) -> Camera {
        // TODO Use FoV to calculate the rectangle
        Camera::default()
    }
}