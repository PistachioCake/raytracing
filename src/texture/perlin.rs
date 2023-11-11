use glamour::Vector3;
use rand::{distributions::Uniform, rngs::ThreadRng, thread_rng, Rng};

use crate::units::{Color, Point, TexCoord};

use super::Texture;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

struct Perlin {
    ran_vec: [Vector3; Self::POINT_COUNT],
    x: [u8; Self::POINT_COUNT],
    y: [u8; Self::POINT_COUNT],
    z: [u8; Self::POINT_COUNT],
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: TexCoord, point: Point) -> Color {
        let point = (point.to_vector() * self.scale).to_point();
        // Color::ONE * (self.noise.turb(point, 7))
        Color::ONE * 0.5 * (1.0 + f32::sin(1.0 + point.z + 10.0 * self.noise.turb(point, 7)))
    }
}

impl Perlin {
    // POINT_COUNT must be at most u8::MAX (or whatever type is in Perlin.{x, y, z})
    pub const POINT_COUNT: usize = 256;

    fn new() -> Self {
        let mut rng = thread_rng();

        let mut this = Perlin {
            ran_vec: [Vector3::ZERO; Self::POINT_COUNT],
            x: [0; Self::POINT_COUNT],
            y: [0; Self::POINT_COUNT],
            z: [0; Self::POINT_COUNT],
        };

        let range = Uniform::new(-1.0, 1.0);
        for vec in this.ran_vec.iter_mut() {
            *vec =
                Vector3::new(rng.sample(range), rng.sample(range), rng.sample(range)).normalize();
            // *vec = random_unit_vector().cast();
        }

        Self::generate_perm(&mut this.x, &mut rng);
        Self::generate_perm(&mut this.y, &mut rng);
        Self::generate_perm(&mut this.z, &mut rng);

        this
    }

    fn generate_perm(p: &mut [u8; Self::POINT_COUNT], rng: &mut ThreadRng) {
        for (i, p) in p.iter_mut().enumerate() {
            *p = i as _;
        }

        let range = Uniform::new(0, Self::POINT_COUNT);

        for ix in (0..Self::POINT_COUNT).rev() {
            let target = rng.sample(range);
            p.swap(ix, target);
        }
    }

    fn noise(&self, p: Point) -> f32 {
        let [(i, u), (j, v), (k, w)] = p
            .as_array()
            .map(|coord| (coord.floor() as isize as usize, coord - coord.floor()));

        // let [u, v, w] = [0.0; 3];
        let [uu, vv, ww] = [u, v, w].map(|weight| weight * weight * (3.0 - 2.0 * weight));

        let mut res = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let c = self.ran_vec[(self.x[(i + di) & 255]
                        ^ self.y[(j + dj) & 255]
                        ^ self.z[(k + dk) & 255])
                        as usize];

                    let [di, dj, dk] = [di, dj, dk].map(|d| d as f32);
                    let weights = Vector3::new(u - di, v - dj, w - dk);
                    // let weights = Vector3::new(1.0, 0.0, 0.0);

                    res += c.dot(weights)
                        * ((di * uu) + (1.0 - di) * (1.0 - uu))
                        * ((dj * vv) + (1.0 - dj) * (1.0 - vv))
                        * ((dk * ww) + (1.0 - dk) * (1.0 - ww));
                }
            }
        }

        res
    }

    fn turb(&self, mut point: Point, depth: usize) -> f32 {
        let mut weight = 1.0;
        (0..depth)
            .map(|_| {
                let contrib = weight * self.noise(point);
                weight /= 2.0;
                point = (point.to_vector() * 2.0).to_point();
                contrib
            })
            .sum::<f32>()
            .abs()
    }
}
