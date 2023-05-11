use serde::Deserialize;
use std::fmt;
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
}

#[derive(Default)]
pub struct PrimitiveVec(pub Vec<Box<dyn Primitive>>);

#[derive(Debug, Deserialize)]
enum PrimitiveType {
    Sphere,
    Plane,
    Cylindre,
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
                let mut primitives = Vec::new();

                while let Some((typ, data)) = seq.next_element::<(PrimitiveType, serde_yaml::Value)>()? {
                    let primitive = match typ {
                        PrimitiveType::Sphere => Sphere::create_sphere_from_serde::<A>(data)?,
                        PrimitiveType::Plane => Plane::create_plane_from_serde::<A>(data)?,
                        PrimitiveType::Cylindre => Cylindre::create_cylindre_from_serde::<A>(data)?,
                    };
                    primitives.push(primitive);
                }
                Ok(PrimitiveVec(primitives))
            }
        }
        deserializer.deserialize_seq(PrimitiveVecVisitor)
    }
}
