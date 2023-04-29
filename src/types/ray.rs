use crate::types::point::Point3D;
use crate::types::vector::Vector3D;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}
