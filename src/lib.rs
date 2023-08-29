use std::io::Write;

use glamour::{Unit, Vector3};

pub struct ColorU;
impl Unit for ColorU {
    type Scalar = f32;
}

pub type Color = Vector3<ColorU>;

pub fn write_color(c: Color, out: &mut dyn Write) {
    let floats = c.as_array();
    let ints = floats.map(|f| (f * 255.999).floor() as u32);

    write!(out, "{} {} {}\n", ints[0], ints[1], ints[2]).unwrap()
}
