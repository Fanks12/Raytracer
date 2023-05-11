use crate::types::primitives::Primitive;

enum RotAxis {
    X,
    Y,
    Z
}

struct Rotation {
    axis : RotAxis,
    angle : f64,
}

impl Rotation {
    fn apply_transform(&self, primitive: Box<dyn Primitive>) {
        
    }
}
