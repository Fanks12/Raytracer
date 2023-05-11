mod core;
mod types;

use std::error;
use crate::core::options::{check_opts};

use crate::core::tracer::raytrace;

fn main() -> Result<(), Box<dyn error::Error>> {
    match check_opts() {
        Ok(opts_args) => {
            if opts_args.is_none() {
                return Ok(());
            }
            raytrace(opts_args.unwrap())
        }
        Err(err) => {
            Err(Box::try_from(format!("Error: {}", err)).unwrap())
        }
    }
}
