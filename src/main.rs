mod core;
mod types;

use std::error;
use crate::core::options::{check_opts, OptsResult};

use crate::core::tracer::raytrace;

fn main() -> Result<(), Box<dyn error::Error>> {
    match check_opts() {
        OptsResult::Ok(opts_args) => {
            raytrace(opts_args)
        }
        OptsResult::Help => {
            Ok(())
        }
        OptsResult::Err => {
            // TODO: Fix error catch and return 84
            std::process::exit(84);
        }
    }
}
