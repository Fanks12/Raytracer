use crate::types::base::point::Point3D;
use crate::types::base::vector::Vector3D;
use crate::types::base::color::Color;
use crate::types::primitives::Primitive;
use crate::types::base::lightpoint::LightPoint;

#[derive(Default, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
    pub hits: Vec<(Point3D, Color)>,
    pub outputs: Vec<Color>
}

impl Ray {
    pub fn first_hit(&mut self, elements: &Vec<Box<dyn Primitive>>, lights: &Vec<LightPoint>) -> Color {
        for element in elements {
            element.intersects(self);
        }
        for light in lights {
            self.outputs.push(light.brightness(self.closest(), elements));
        }
        self.brighter()
    }

    pub fn closest(&self) -> (Point3D, Color) {
        let mut dist: f64 = -1.0;
        let mut closest = (Point3D {..Default::default()}, Color {..Default::default()});
        for element in &self.hits {
            if dist < 0.0 || self.origin.distance(element.0) < dist {
                dist = self.origin.distance(element.0);
                closest = *element;
            }
        }
        closest
    }

    fn brighter(&self) -> Color {
        let mut res: Color = Color{..Default::default()};
        for color in &self.outputs {
            if color.sum() > res.sum() {
                res = *color;
            }
        }
        res
    }
}
