use std::io::Write;

use glamour::{Point2, Point3, Unit, Vector3};
use rand::{distributions::Uniform, thread_rng, Rng};

pub struct ColorSpace;
impl Unit for ColorSpace {
    type Scalar = f32;
}

pub type Color = Vector3<ColorSpace>;

pub struct WorldSpace;
impl Unit for WorldSpace {
    type Scalar = f32;
}

pub type Point = Point3<WorldSpace>;
pub type Vector = Vector3<WorldSpace>;

pub struct TextureSpace;
impl Unit for TextureSpace {
    type Scalar = f32;
}

pub type TexCoord = Point2<TextureSpace>;

pub fn write_color(out: &mut dyn Write, c: Color) {
    let floats = c.as_array();
    let ints = floats
        .map(f32::sqrt) // linear to gamma
        .map(|f| (f.clamp(0., 1.) * 255.999).floor() as u32) // convert to integer in [0, 255]
        ;

    writeln!(out, "{} {} {}", ints[0], ints[1], ints[2]).unwrap()
}

pub fn random_in_unit_sphere() -> Vector {
    let mut rng = thread_rng();
    let distr = Uniform::new(-1., 1.);
    loop {
        let v = Vector::new(rng.sample(distr), rng.sample(distr), rng.sample(distr));
        if v.length_squared() < 1. {
            break v;
        }
    }
}

pub fn random_unit_vector() -> Vector {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: &Vector) -> Vector {
    let v = random_unit_vector();
    if v.dot(*normal) > 0. {
        v
    } else {
        -v
    }
}

pub fn random_in_unit_disk() -> Vector {
    let mut rng = thread_rng();
    let distr = Uniform::new(-1., 1.);
    loop {
        let v = Vector::new(rng.sample(distr), rng.sample(distr), 0.);
        if v.length_squared() < 1. {
            break v;
        }
    }
}

pub fn reflect(v: &Vector, n: &Vector) -> Vector {
    *v - *n * 2. * v.dot(*n)
}

pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: f32) -> Vector {
    let cos_theta = n.dot(-*uv).min(1.);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_para = *n * -(1. - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_para
}
