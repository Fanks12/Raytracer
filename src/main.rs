mod core;
mod opts;
mod types;

use std::error;
use crate::core::tracer::raytrace;
use crate::opts::check_opts;
use crate::opts::OptsResult;

fn main() -> Result<(), Box<dyn error::Error>> {
    match check_opts() {
        OptsResult::Ok(opts_args) => {
            return raytrace();
        }
        OptsResult::Help => {
            std::process::exit(0);
        }
        OptsResult::Err => {
            std::process::exit(84);
        }
    }
}
