mod core;
mod types;

use std::env;
use std::error;

use crate::core::tracer::raytrace;

fn main() -> Result<(), Box<dyn error::Error>>
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Number of arguments");
        std::process::exit(84);
    }
    if args[1] == "--help" {
        return print_help(&args[0])
    }
    raytrace()
}

fn print_help(binary: &String) -> Result<(), Box<dyn error::Error>> {
    println!("USAGE: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
    Ok(())
}
