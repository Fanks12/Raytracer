use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;
use crate::types::base::color::Color;
use crate::types::primitives::Primitive;

#[derive(Default, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
    pub hits: Vec<(Point3D, Color)>
}

impl Ray {
    pub fn first_hit(&mut self, elements: &Vec<Box<dyn Primitive>>) -> Color {
        for element in elements {
            element.intersects(self);
        }
        self.closest().1
    }

    fn closest(&self) -> (Point3D, Color) {
        let mut dist: f64 = -1.0;
        let mut closest = (Point3D {..Default::default()}, Color {..Default::default()});
        for element in &self.hits {
            if dist < 0.0 || self.origin.dist(element.0) < dist {
                dist = self.origin.dist(element.0);
                closest = *element;
            }
        }
        closest
    }
}
