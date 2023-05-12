use serde::Deserialize;
use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;
use crate::types::primitives::Primitive;
use crate::types::base::color::Color;
use crate::types::base::ray::Ray;
use crate::types::transformations::rotation::Rotation;


#[derive(Clone, Copy, Deserialize)]
pub struct Cylindre {
    radius : f64,
    height : f64,
    center : Point3D,
    pub body_vector : Vector3D,
    color: Color,
}

impl<'de> Cylindre {
    pub fn create_cylindre_from_serde<A>(data : serde_yaml::Value) -> Result<Box<dyn Primitive>, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let cylindre: Cylindre;
        match serde_yaml::from_value(data) {
            Ok(obj) => cylindre = obj,
            Err(err) => {
                return Err(serde::de::Error::custom(format!("Failed to deserialize cylindre: {}", err)));
            }
        }
        Ok(Box::new(cylindre) as Box<dyn Primitive>)
    }
}

impl Cylindre {
    fn get_normalised_h(&self) -> Vector3D {
        let top_origin: Point3D = Point3D { x: self.center.x, y: self.center.y, z: self.center.z + self.height };
        let h: Vector3D = Vector3D { x: self.center.x - top_origin.x  , y: self.center.y - top_origin.y, z: self.center.z - top_origin.z };
        h.normalize()
    }
}
impl Primitive for Cylindre {


    fn intersects(&self, ray: &mut Ray) {
        let norm_h = self.get_normalised_h();
        let w: Vector3D = Vector3D { x: ray.origin.x - self.center.x, y: ray.origin.y - self.center.y, z: ray.origin.z - self.center.z };
        let a: f64 = ray.direction.dot_product(ray.direction) - (ray.direction.dot_product(norm_h)).powi(2);
        let b: f64 = 2.0 * (ray.direction.dot_product(w) - ray.direction.dot_product(norm_h) * w.dot_product(norm_h));
        let c: f64 = w.dot_product(w) - w.dot_product(norm_h).powi(2) - self.radius.powi(2);
        let d: f64 = b.powi(2) - 4.0 * a * c;

        if d == 0.0 {
            let t: f64 = -b / (2.0 * a);
            ray.hits.push((Point3D { x: ray.origin.x + ray.direction.x * t, y: ray.origin.y + ray.direction.y * t, z: ray.origin.z + ray.direction.z * t}, self.color));
        } else if d > 0.0 {
            let tun: f64 = (-b + d.sqrt()) / (2.0 * a);
            let tdeux: f64 = (-b - d.sqrt()) / (2.0 * a);
            ray.hits.push((Point3D { x: ray.origin.x + ray.direction.x * tun, y: ray.origin.y + ray.direction.y * tun, z: ray.origin.z + ray.direction.z * tun}, self.color));
            ray.hits.push((Point3D { x: ray.origin.x + ray.direction.x * tdeux, y: ray.origin.y + ray.direction.y * tdeux, z: ray.origin.z + ray.direction.z * tdeux}, self.color));
        }
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn rotate_x(&self, rota : Rotation) {
        println!("rotate x for {} degrees", rota.angle)
        // self.body_vector = Vector3D { x: 0.0 , y: rota.angle.cos() * self.body_vector.y, z: -rota.angle.sin() * self.body_vector.z}
    }

    fn rotate_y(&self, rota : Rotation) {
        println!("rotate y for {} degrees", rota.angle)
        // self.body_vector = Vector3D { x: 0.0 , y: rota.angle.cos() * self.body_vector.y, z: -rota.angle.sin() * self.body_vector.z}
    }
    fn rotate_z(&self, rota : Rotation) {
        println!("rotate z for {} degrees", rota.angle)
    }
}
