use std::io::Read;
use std::error;
use serde::Deserialize;
use crate::types::config::camera::CamConfig;
use crate::types::config::Light;
use crate::types::config::scene::Scene;
use crate::types::primitives::sphere::Sphere;

#[derive(Default, Deserialize)]
pub struct Parser {
    pub lights: Vec<Light>,
    pub camera: CamConfig,
    pub spheres: Vec<Sphere>,
}

pub fn parse_file(file_path: &String)-> Result<Scene, Box<dyn error::Error>>
{
    let path = file_path;
    let mut file = std::fs::File::open(path).map_err(|error| {
        format!("Failed to read file {}: {}", path, error)
    })?;

    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|error| {
        format!("Failed to read file contents: {}", error)
    })?;

    let data: &str = &content;
    let parser : Parser = serde_json::from_str(data).map_err(|error| {
        format!("Failed to convert light : {}", error)
    })?;
    let scene : Scene = Scene::create_from_parser(parser);
    Ok(scene)
}
