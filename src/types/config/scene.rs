use crate::core::parser::Parser;
use crate::types::config::{Light, Resolution};
use crate::types::config::camera::Camera;
use crate::types::primitives::PrimitiveVec;

#[derive(Default)]
pub struct Scene {
    pub camera : Camera,
    pub resolution : Resolution,
    pub primitives : PrimitiveVec,
    pub lights : Vec<Light>,
}

impl Scene {
    pub fn create_from_parser(parser: Parser) -> Self
    {
        Self {
            camera: parser.camera.convert(),
            resolution : parser.camera.resolution,
            primitives: (parser.primitives),
            lights: (parser.lights),
        }
    }
}
