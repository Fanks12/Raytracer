use crate::types::base::ray::Ray;
use self::plane::Plane;
use self::sphere::Sphere;

use serde::Deserialize;
use std::fmt;

pub mod plane;
pub mod sphere;

pub trait Primitive {
    fn intersects(&self, ray: &mut Ray);
}

#[derive(Default)]
pub struct PrimitiveVec(pub Vec<Box<dyn Primitive>>);

#[derive(Debug, Deserialize)]
enum PrimitiveType {
    Sphere,
    Plane,
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
                        PrimitiveType::Sphere => {
                            let sphere: Sphere;
                            match serde_yaml::from_value(data) {
                                Ok(obj) => sphere = obj,
                                Err(err) => {
                                    return Err(serde::de::Error::custom(format!("Failed to deserialize sphere: {}", err)));
                                }
                            }
                            Box::new(sphere) as Box<dyn Primitive>
                        },
                        PrimitiveType::Plane => {
                            let plane : Plane;
                            match serde_yaml::from_value(data) {
                                Ok(obj) => plane = obj,
                                Err(err) => {
                                    return Err(serde::de::Error::custom(format!("Failed to deserialize plane: {}", err)));
                                }
                            }
                            Box::new(plane) as Box<dyn Primitive>
                        },
                    };
                    primitives.push(primitive);
                }
                Ok(PrimitiveVec(primitives))
            }
        }

        deserializer.deserialize_seq(PrimitiveVecVisitor)
    }
}
