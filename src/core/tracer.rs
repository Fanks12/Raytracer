use std::error;
use crate::core::image::write_ppm_file;
use crate::core::options::OptsArgs;
use crate::core::parser::parse_file;
use crate::types::base::color::Color;
use crate::types::config::scene::Scene;

pub fn raytrace(options: OptsArgs) -> Result<(), Box<dyn error::Error>> {
    let result = parse_file(&options.scene_file);
    match result {
        Ok(_) => exec_raytracing(options, result.unwrap()),
        Err(err) => return Err(err),
    }
}

fn exec_raytracing(options: OptsArgs, scene: Scene) -> Result<(), Box<dyn error::Error>> {
    let points_x = scene.resolution.width;
    let points_y = scene.resolution.height;
    let mut colors: Vec<Color> = Vec::with_capacity(points_y * points_x);

    for y in 0..points_y {
        for x in 0..points_x {
            let xf = x as f64 / points_x as f64;
            let yf = y as f64 / points_y as f64;
            let color = scene.camera.ray(xf, yf).first_hit(&scene.primitives.0);

            colors.push(color);
        }
    }
    if options.external_file.is_some() {
        return write_ppm_file(&options.external_file.unwrap(), points_x, points_y, colors);
    }
    // TODO: Graphical implementation
    println!("Graphic visualization not implemented yet, please use --export");
    Ok(())
}
