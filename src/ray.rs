use crate::units::{Point, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direct: Vector,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direct * t
    }
}
