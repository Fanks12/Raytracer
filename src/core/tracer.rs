
use crate::parser::open_file::Scene;
use std::error;

pub fn raytrace(scene : Scene) -> Result<(), Box<dyn error::Error>> {
    let points_x = 60;
    let points_y = 20;
    let mut output: Vec<char> = Vec::with_capacity(points_x * (points_y + 1));

    for y in 1..points_y {
        for x in 1..points_x {
            let xf = x as f64 / points_x as f64;
            let yf = y as f64 / points_y as f64;
            if scene.primitives[0].hits(scene.camera.ray(xf, yf)) {
                output.push(' ');
            } else {
                output.push('#');
            }
        }
        output.push('\n');
    }

    let out: String = output.iter().collect();
    println!("{}", out);
    Ok(())
}
