mod core;
mod opts;
mod types;

use crate::core::tracer::raytrace;
use crate::opts::check_opts;
use crate::opts::OptsResult;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    match check_opts() {
        OptsResult::Ok(opts_args) => {
            return raytrace();
        }
        OptsResult::Help => {
            return Ok(());
        }
        OptsResult::Err => {
            // TODO: Fix error catch and return 84
            std::process::exit(84);
        }
    }
}
