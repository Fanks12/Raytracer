mod core;
mod types;
mod parser;

use std::env;
use std::process::exit;
use std::panic;

use crate::core::tracer::raytrace;
use crate::parser::open_file::parse_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Number of arguments");
        std::process::exit(84);
    }
    if args[1] == "--help" {
        print_help(&args[0]);
    }
    let result = panic::catch_unwind(|| {
        parse_file(&args[1]);
    });
    if result.is_err() {
        exit(84);
    }
    raytrace();
}

fn print_help(binary: &String) {
    println!("USAGE: {} <SCENE_FILE>", binary);
    println!("\tSCENE_FILE: scene configuration");
}
