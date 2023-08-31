use std::io::Write;

use glamour::{Point3, Unit, Vector3};

pub struct ColorSpace;
impl Unit for ColorSpace {
    type Scalar = f32;
}

pub type Color = Vector3<ColorSpace>;

pub fn write_color(out: &mut dyn Write, c: Color) {
    let floats = c.as_array();
    let ints = floats.map(|f| (f.clamp(0., 1.) * 255.999).floor() as u32);

    write!(out, "{} {} {}\n", ints[0], ints[1], ints[2]).unwrap()
}

pub struct WorldSpace;
impl Unit for WorldSpace {
    type Scalar = f32;
}

pub type Point = Point3<WorldSpace>;
pub type Vector = Vector3<WorldSpace>;
