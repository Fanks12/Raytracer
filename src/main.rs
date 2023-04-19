mod core;
mod types;

use std::env;
use crate::core::tracer::raytrace;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Number of arguments");
        std::process::exit(84);
    }
    if args[1] == "--help" {
        print_help(&args[0]);
    }
    raytrace();
}

fn print_help(binary: &String) {
    println!("USAGE: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}
