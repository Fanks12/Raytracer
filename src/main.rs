//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-florian.lauch
// File description:
// main
//

mod vector3d;

use vector3d::Vector3D;

fn main() {
    let vector = Vector3D::new(2.0, 3.0, 4.0);
    let vector2 = Vector3D::default();

    println!("Vector3D = {:?}", vector);
}
