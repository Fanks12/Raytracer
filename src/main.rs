mod core;
mod file;
mod types;

use std::env;
use crate::core::tracer::raytrace;
use crate::file::open_scene;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut scene_contents: String = String::new();

    if args.len() != 2 {
        eprintln!("Error: Number of arguments");
        std::process::exit(84);
    }
    if args[1] == "--help" {
        print_help(&args[0]);
    }
    if !open_scene(&args[1], &mut scene_contents) {
        std::process::exit(84);
    }
    raytrace();
}

fn print_help(binary: &String) {
    println!("USAGE: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}
