use crate::types::point::Point3D;
use crate::types::ray::Ray;
use crate::types::rectangle::Rectangle3D;
use serde::Deserialize;

#[derive(Default, Deserialize)]
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
