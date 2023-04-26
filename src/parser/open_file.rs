//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-florian.lauch
// File description:
// open_file
//

// use crate::types::camera::Camera;
use crate::types::vector::Vector3D;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::Result;
// use crate::types::traits::Hitable;

#[derive(Default, Serialize, Deserialize)]
pub struct Light {
    pub pos : Vector3D,
    pub angle : f32,
}

// , prims : &Vec<Box<dyn Hitable>>, lights: &Vec<Light>

pub fn parse_file(file_path: &String)-> Result<()>
{
    let path = file_path;
    let mut file = std::fs::File::open(path).unwrap_or_else(|error| {
        panic!("Failed to read file {}: {}", path, error);
    });

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_else(|error| {
        panic!("Failed to read file contents: {}", error);
    });

    let data: &str = &content;
    let light : Light = serde_json::from_str(data)?;
    println!("pos.x = {} angle = {}", light.pos.x, light.angle);
    Ok(())
}
