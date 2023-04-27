//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-florian.lauch
// File description:
// open_file
//

use crate::types::{camera::Camera, traits::Hitable};
use crate::types::vector::Vector3D;
use crate::types::sphere::Sphere;

use std::io::Read;
use std::error;
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Light {
    pub pos : Vector3D,
    pub angle : f32,
}
#[derive(Default)]
pub struct Scene {
    pub lights : Vec<Light>,
    pub camera : Camera,
    pub primitives : Vec<Box<dyn Hitable>>,
}

#[derive(Default, Deserialize)]
pub struct Parser {
    pub lights : Vec<Light>,
    pub camera : Camera,
    pub spheres : Vec<Sphere>,
}

impl Scene {
    pub fn create_from_parser(parser : Parser) -> Self
    {
        let mut vect : Vec<Box<dyn Hitable>> = Vec::default();
        for val in parser.spheres.iter() {
            vect.push(Box::new(*val));
        }
        Self { lights: (parser.lights), camera: (parser.camera),
        primitives: (vect) }
    }
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
    println!("pos.x = {} angle = {}", scene.lights[0].pos.x, scene.lights[0].angle);
    Ok(scene)
}
