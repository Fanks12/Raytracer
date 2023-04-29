use crate::core::parser::Parser;
use crate::types::config::{Light, Resolution};
use crate::types::config::camera::Camera;
use crate::types::primitives::Primitive;

#[derive(Default)]
pub struct Scene {
    pub camera : Camera,
    pub resolution : Resolution,
    pub primitives : Vec<Box<dyn Primitive>>,
    pub lights : Vec<Light>,
}

impl Scene {
    pub fn create_from_parser(parser: Parser) -> Self
    {
        let mut vect : Vec<Box<dyn Primitive>> = Vec::default();
        for val in parser.spheres.iter() {
            vect.push(Box::new(*val));
        }
        Self {
            camera: parser.camera.convert(),
            resolution : parser.camera.resolution,
            primitives: (vect),
            lights: (parser.lights),
        }
    }
}
