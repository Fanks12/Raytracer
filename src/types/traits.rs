use crate::types::ray::Ray;

pub trait Primitives {
    fn intersects(&self, ray: &mut Ray);
}
