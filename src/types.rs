pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait HasLength {
    fn length(&self) -> f64;
}

impl HasLength for Vector3D {
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
