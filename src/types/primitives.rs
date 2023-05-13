use serde::{Deserialize};
use std::fmt;

use crate::types::transformations::rotation::Rotation;
use crate::types::base::color::Color;
use crate::types::base::ray::Ray;
use crate::types::primitives::cylindre::Cylindre;
use crate::types::primitives::plane::Plane;
use crate::types::primitives::sphere::Sphere;

pub mod plane;
pub mod sphere;
pub mod cylindre;

pub trait Primitive {
    fn intersects(&self, ray: &mut Ray);
    fn get_color(&self) -> &Color;
    fn rotate_x(&mut self, rota : Rotation);
    fn rotate_y(&mut self, rota : Rotation);
    fn rotate_z(&mut self, rota : Rotation);
}

#[derive(Default)]
pub struct PrimitiveVec(pub Vec<Box<dyn Primitive>>);

#[derive(Debug, Deserialize)]
enum PrimitiveType {
    Sphere,
    Plane,
    Cylindre,
    Rotation,
}

impl<'de> serde::Deserialize<'de> for PrimitiveVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PrimitiveVecVisitor;

        impl<'de> serde::de::Visitor<'de> for PrimitiveVecVisitor {
            type Value = PrimitiveVec;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A struct PrimitiveVec")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut primitives: Vec<Box<dyn Primitive>> = Vec::new();
                let mut primitive: Option<Box<dyn Primitive>> = None;

                while let Some((typ, data)) = seq.next_element::<(PrimitiveType, serde_yaml::Value)>()? {
                    match typ {
                        PrimitiveType::Sphere => primitive = Some(Sphere::create_sphere_from_serde::<A>(data)?),
                        PrimitiveType::Plane => primitive = Some(Plane::create_plane_from_serde::<A>(data)?),
                        PrimitiveType::Cylindre => primitive = Some(Cylindre::create_cylindre_from_serde::<A>(data)?),
                        PrimitiveType::Rotation =>  match Rotation::rotate_primitive::<A>(primitives.last_mut(), data) {
                            Ok(()) => continue,
                            Err(err) => return Err(err)
                            }
                    };
                    match primitive {
                        Some(primitive) => primitives.push(primitive),
                        None => continue
                    }
                }
                Ok(PrimitiveVec(primitives))
            }
        }
        deserializer.deserialize_seq(PrimitiveVecVisitor)
    }
}
