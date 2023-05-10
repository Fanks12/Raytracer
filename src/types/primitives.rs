use crate::types::base::color::Color;
use crate::types::base::ray::Ray;

pub mod plane;
pub mod sphere;
pub mod cone;

pub trait Primitive {
    fn intersects(&self, ray: &mut Ray);
    fn get_color(&self) -> &Color;
}
