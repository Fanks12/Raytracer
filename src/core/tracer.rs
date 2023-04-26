use crate::types::camera::Camera;
use crate::types::point::Point3D;
use crate::types::rectangle::Rectangle3D;
use crate::types::sphere::Sphere;
use crate::types::traits::Hitable;
use crate::types::vector::Vector3D;
use std::error;

pub fn raytrace() -> Result<(), Box<dyn error::Error>>
{
    let eye: Point3D = Point3D { x: 0.0, y: 0.0, z: -5.0 };
    let screen_origin: Point3D = Point3D { x: -5.0, y: -5.0, z: 0.0 };
    let screen_left: Vector3D = Vector3D { x: 10.0, y: 0.0, z: 0.0 };
    let screen_bottom: Vector3D = Vector3D { x: 0.0, y: 10.0, z: 0.0 };
    let screen: Rectangle3D = Rectangle3D { origin: screen_origin, bottom_side: screen_bottom, left_side: screen_left };
    let camera: Camera = Camera { origin: eye, screen };
    let element: Sphere = Sphere { center: Default::default(), radius: 5.0 };

    let points_x = 60;
    let points_y = 20;
    let mut output: Vec<char> = Vec::with_capacity(points_x * (points_y + 1));

    for y in 1..points_y {
        for x in 1..points_x {
            let xf = x as f64 / points_x as f64;
            let yf = y as f64 / points_y as f64;
            if element.hits(camera.ray(xf, yf)) {
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
