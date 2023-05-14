use serde::Deserialize;

use crate::types::primitives::Primitive;

#[derive(Deserialize)]
pub enum RotAxis {
    X,
    Y,
    Z
}

#[derive(Deserialize)]
pub struct Rotation {
    pub axis : RotAxis,
    pub angle : f64,
}

impl<'de> Rotation {
    pub fn rotate_primitive<'a, A>(obj: Option<&'a mut Box<dyn Primitive + Send + Sync>>, data : serde_yaml::Value) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let primitive : &'a mut Box<dyn Primitive + Send + Sync>;

        match obj {
            Some(p) => primitive = p,
            None => return Err(serde::de::Error::custom(format!("Invalid transformation, specify primitive first")))
        }
        let rotation : Rotation;
        match serde_yaml::from_value(data) {
            Ok(value) => rotation = value,
            Err(err) => {
                return Err(serde::de::Error::custom(format!("Failed to deserialize rotation: {}", err)));
            }
        }
        rotation.apply_transform(primitive);
        Ok(())
    }
}

impl Rotation {
    fn apply_transform(self, primitive : &mut Box<dyn Primitive + Send + Sync>) {
            match self.axis {
                RotAxis::X => primitive.rotate_x(self),
                RotAxis::Y => primitive.rotate_y(self),
                RotAxis::Z => primitive.rotate_z(self)
            }
        }
}
