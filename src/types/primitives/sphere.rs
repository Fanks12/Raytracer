use crate::types::point::Point3D;
use crate::types::ray::Ray;
use crate::types::traits::Hitable;

pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hits(&self, ray: Ray) -> bool {
        let a: f64 = ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b: f64 = 2.0 * (ray.origin.x * ray.direction.x + ray.origin.y * ray.direction.y + ray.origin.z * ray.direction.z - self.center.x * ray.direction.x - self.center.y * ray.direction.y - self.center.z * ray.direction.z);
        let c: f64 = ray.origin.x.powi(2) + ray.origin.y.powi(2) + ray.origin.z.powi(2) + self.center.x.powi(2) + self.center.y.powi(2) + self.center.z.powi(2) - 2.0 * (ray.origin.x * self.center.x + ray.origin.y * self.center.y + ray.origin.z * self.center.z) - self.radius.powi(2);
        let d: f64 = b.powi(2) - 4.0 * a * c;

        d >= 0.0
    }
    fn intersect_point(&self, ray: Ray) -> Vec<Point3D> {
        let tab = Vec::new();
        // TODO: Implementation
        // if d == 0.0 {
        //     let t: f64 = -b / 2 * a
        //     tab.push(Point3D { x: ray.origin.x + ray.direction.x * t, y: ray.origin.y + ray.direction.y * t, z: ray.origin.z + ray.direction.z * t});
        // } else if d > 0.0 {
        //     let tun: f64 = -b + d.sqrt() / 2 * a
        //     let tdeux: f64 = -b - d.sqrt() / 2 * a
        //     tab.push(Point3D { x: ray.origin.x + ray.direction.x * tun, y: ray.origin.y + ray.direction.y * tun, z: ray.origin.z + ray.direction.z * tun});
        //     tab.push(Point3D { x: ray.origin.x + ray.direction.x * tdeux, y: ray.origin.y + ray.direction.y * tdeux, z: ray.origin.z + ray.direction.z * tdeux});
        // }
        tab
    }
}
