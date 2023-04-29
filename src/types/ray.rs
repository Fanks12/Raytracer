use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}
