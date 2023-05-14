use std::error;
use crate::core::image::write_ppm_file;
use crate::core::options::{check_opts, OptsArgs};
use crate::core::parser::parse_file;
use crate::core::renderer::Renderer;
use crate::types::config::scene::Scene;

pub fn raytracer() -> Result<Option<Renderer>, Box<dyn error::Error>> {
    match check_opts() {
        Ok(opts_args) => {
            if opts_args.is_none() {
                return Ok(None);
            }
            raytrace(opts_args.unwrap())
        }
        Err(err) => {
            Err(Box::try_from(format!("Error: {}", err)).unwrap())
        }
    }
}

pub fn raytrace(options: OptsArgs) -> Result<Option<Renderer>, Box<dyn error::Error>> {
    let result = parse_file(&options.scene_file);
    match result {
        Ok(_) => exec_raytracing(options, result.unwrap()),
        Err(err) => Err(err),
    }
}

fn exec_raytracing(options: OptsArgs, scene: Scene) -> Result<Option<Renderer>, Box<dyn error::Error>> {
    let points_x = scene.resolution.width;
    let points_y = scene.resolution.height;
    let mut renderer = Renderer::new(scene);

    if options.external_file.is_some() {
        renderer.render();
        let bytes = renderer.get_raw_colors(false);
        renderer.terminate();
        match write_ppm_file(&options.external_file.unwrap(), points_x, points_y, bytes) {
            Ok(_) => return Ok(None),
            Err(err) => return Err(err)
        }
    }
    Ok(Some(renderer))
}
