use crate::types::camera::Camera;
use crate::types::point::Point3D;
use crate::types::rectangle::Rectangle3D;
use crate::types::sphere::Sphere;
use crate::types::plane::Plane;
use crate::types::traits::Primitives;
use crate::types::vector::Vector3D;
use crate::types::color::Color;

pub fn raytrace() {
    let eye: Point3D = Point3D { x: 0.0, y: 0.0, z: -5.0 };
    let screen_origin: Point3D = Point3D { x: -5.0, y: -5.0, z: 0.0 };
    let screen_left: Vector3D = Vector3D { x: 10.0, y: 0.0, z: 0.0 };
    let screen_bottom: Vector3D = Vector3D { x: 0.0, y: 10.0, z: 0.0 };
    let sphere_center: Point3D = Point3D { x: 0.0, y: 0.0, z: -10.0 };
    let screen: Rectangle3D = Rectangle3D { origin: screen_origin, bottom_side: screen_bottom, left_side: screen_left };
    let camera: Camera = Camera { origin: eye, screen };
    let plane: Plane = Plane { axis: 'X', padding: 1.0, color: Color {red: 255, green: 0, blue: 0} };
    let sphere: Sphere = Sphere { center: sphere_center, radius: 2.0, color: Color {red: 155, green: 0, blue: 0}};
    let element: Vec<&dyn Primitives> = vec![&plane, &sphere];

    let points_x = 192;
    let points_y = 108;
    let mut output: Vec<char> = Vec::with_capacity(points_x * (points_y + 1));

    for y in 1..points_y {
        for x in 1..points_x {
            let xf = x as f64 / points_x as f64;
            let yf = y as f64 / points_y as f64;
            let color = camera.ray(xf, yf).first_hit(element.clone());

            if color.red == 255 {
                output.push('=');
            } else if color.red == 155 {
                output.push('O');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    let out: String = output.iter().collect();
    println!("{}", out);
}
