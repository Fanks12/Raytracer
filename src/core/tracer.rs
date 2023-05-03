use std::error;
use crate::core::options::OptsArgs;
use crate::core::parser::parse_file;
use crate::types::config::scene::Scene;

pub fn raytrace(options: OptsArgs) -> Result<(), Box<dyn error::Error>> {
    let result = parse_file(&options.scene_file);
    match result {
        Ok(_) => exec_raytracing(result.unwrap()),
        Err(err) => return Err(err),
    }
}

fn exec_raytracing(scene: Scene) -> Result<(), Box<dyn error::Error>> {
    let points_x = scene.resolution.width;
    let points_y = scene.resolution.height;
    let mut output: Vec<char> = Vec::with_capacity(points_x * (points_y + 1));

    // for y in 0..points_y {
    //     if y > 0 {
    //         output.push('\n');
    //     }

    //     for x in 0..points_x {
    //         let xf = x as f64 / points_x as f64;
    //         let yf = y as f64 / points_y as f64;
    //         if scene.primitives[0].hits(scene.camera.ray(xf, yf)) {
    //             output.push(' ');
    //         } else {
    //             output.push('#');
    //         }
    //     }
    // }
    for y in 0..points_y {
        if y > 0 {
            output.push('\n');
        }
        for x in 0..points_x {
            let xf = x as f64 / points_x as f64;
            let yf = y as f64 / points_y as f64;
            let color = scene.camera.ray(xf, yf).first_hit(&scene.primitives);

            if color.red == 255 {
                output.push('=');
            } else if color.red == 155 {
                output.push('O');
            } else {
                output.push(' ');
            }
        }
    }

    let out: String = output.iter().collect();
    println!("{}", out);
    Ok(())
}
