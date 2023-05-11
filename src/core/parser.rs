use serde::Deserialize;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::fmt;
use crate::types::config::camera::CamConfig;
use crate::types::config::Light;
use crate::types::config::scene::Scene;
use crate::types::primitives::PrimitiveVec;


#[derive(Debug)]
struct ExtensionError(String);

impl fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Extension Error: {}", self.0)
    }
}

impl Error for ExtensionError {}

#[derive(Default, Deserialize)]
pub struct Parser {
    pub lights: Vec<Light>,
    pub camera: CamConfig,
    pub primitives: PrimitiveVec,
}

pub enum FileType {
    Json,
    Yaml,
    Unknown
}

pub fn parse_json(content : &str) -> Result<Scene, Box<dyn Error>>
{
    let data: &str = content;
    let parser : Parser = serde_json::from_str(data).map_err(|error| {
        format!("Failed to convert scene : {}", error)
    })?;
    let scene : Scene = Scene::create_from_parser(parser);

    Ok(scene)
}

pub fn parse_yaml(content : &str) -> Result<Scene, Box<dyn Error>>
{
    let data: &str = content;
    let parser : Parser = serde_yaml ::from_str(data).map_err(|error| {
        format!("Failed to convert scene : {}", error)
    })?;
    let scene : Scene = Scene::create_from_parser(parser);

    Ok(scene)
}

pub fn select_file_type(file_path: &String) -> FileType
{
    match Path::new(&file_path).extension() {
        Some(ext) => {
            if ext == "json" {
                FileType::Json
            } else if ext == "yaml" {
                FileType::Yaml
            } else {
                FileType::Unknown
            }
        }
        None => FileType::Unknown,
    }
}

pub fn parse_file(file_path: &String) -> Result<Scene, Box<dyn Error>>
{
    let path = file_path;
    let mut file = std::fs::File::open(path).map_err(|error| {
        format!("Failed to read file {}: {}", path, error)
    })?;

    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|error| {
        format!("Failed to read file contents: {}", error)
    })?;
    match select_file_type(file_path) {
        FileType::Json => parse_json(&content),
        FileType::Yaml => parse_yaml(&content),
        FileType::Unknown => Err(Box::new(ExtensionError("Unknown extension".into()))),
    }
}
