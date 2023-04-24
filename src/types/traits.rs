use crate::types::ray::Ray;

pub trait Hitable {
    fn hits(&self, ray: Ray) -> bool;
}
