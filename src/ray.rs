use crate::units::{Point, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direct: Vector,
    pub time: f32,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direct * t
    }
}
