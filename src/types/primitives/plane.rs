use serde::Deserialize;
use crate::types::base::point::Point3D;
use crate::types::primitives::Primitive;
use crate::types::base::color::Color;
use crate::types::base::ray::Ray;
use crate::types::transformations::rotation::Rotation;
use crate::types::base::vector::Vector3D;

#[derive(Deserialize, Copy, Clone, Default)]
pub struct Plane {
    pub point: Point3D,
    pub norm: Vector3D,
    pub color: Color,
}

impl<'de> Plane {
    pub fn create_plane_from_serde<A>(data : serde_yaml::Value) -> Result<Box<dyn Primitive>, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let plane: Plane = match serde_yaml::from_value(data) {
            Ok(obj) => obj,
            Err(err) => {
                return Err(serde::de::Error::custom(format!("Failed to deserialize plane: {}", err)));
            }
        };
        Ok(Box::new(plane) as Box<dyn Primitive>)
    }
}

impl Primitive for Plane {
    fn intersects(&self, ray: &mut Ray) {
        let cross = ray.direction.dot_product(self.norm);

        if cross != 0.0 {
            let coef = (ray.origin - self.point).to_vec().dot_product(self.norm) / cross;

            if coef > 0.0 {
                ray.hits.push((ray.origin + ray.direction * coef, self.color));
            }
        }

        /*let a: f64 = ray.origin.axis_value(self.axis);
        let b: f64 =  ray.origin.axis_value(self.axis) + ray.direction.axis_value(self.axis);
        let diffa: f64 = a + self.padding;
        let diffb: f64 = b + self.padding;
        if diffa.abs() > diffb.abs() {
            let ratio: f64 = (-self.padding - ray.origin.axis_value(self.axis)) / ray.direction.axis_value(self.axis);
            ray.hits.push((ray.origin + ray.direction * ratio, self.color));
        }*/
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn rotate_x(&self, rota : Rotation) {
        println!("rotate x")
        // self.body_vector = Vector3D { x: 0.0 , y: rota.angle.cos() * self.body_vector.y, z: -rota.angle.sin() * self.body_vector.z}
    }

    fn rotate_y(&self, rota : Rotation) {
        println!("rotate y")
        // self.body_vector = Vector3D { x: 0.0 , y: rota.angle.cos() * self.body_vector.y, z: -rota.angle.sin() * self.body_vector.z}
    }
    fn rotate_z(&self, rota : Rotation) {
        println!("rotate z")
    }
}
