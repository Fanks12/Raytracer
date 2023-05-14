use crate::types::primitives::Primitive;

pub mod rotation;

pub trait Transformations {
    fn apply_transform(&self, primitive: Box<dyn Primitive>);
}
