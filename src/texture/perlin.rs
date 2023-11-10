use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::units::{Color, Point, TexCoord};

use super::Texture;

pub struct NoiseTexture {
    noise: Perlin,
}

struct Perlin {
    ran_floats: [f32; Self::POINT_COUNT],
    x: [u8; Self::POINT_COUNT],
    y: [u8; Self::POINT_COUNT],
    z: [u8; Self::POINT_COUNT],
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: TexCoord, point: Point) -> Color {
        Color::ONE * self.noise.noise(point)
    }
}

impl Perlin {
    // POINT_COUNT must be at most u8::MAX (or whatever type is in Perlin.{x, y, z})
    pub const POINT_COUNT: usize = 256;

    fn new() -> Self {
        let mut rng = thread_rng();

        let mut this = Perlin {
            ran_floats: [0.0; Self::POINT_COUNT],
            x: [0; Self::POINT_COUNT],
            y: [0; Self::POINT_COUNT],
            z: [0; Self::POINT_COUNT],
        };

        for float in this.ran_floats.iter_mut() {
            *float = rng.gen();
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

        let range = rand::distributions::Uniform::new(0, Self::POINT_COUNT);

        for ix in (0..Self::POINT_COUNT).rev() {
            let target = rng.sample(range);
            p.swap(ix, target);
        }
    }

    fn noise(&self, p: Point) -> f32 {
        let [i, j, k] = p
            .as_array()
            .map(|coord| ((4.0 * coord).floor() as i32 & 255) as usize);

        self.ran_floats[(self.x[i] ^ self.y[j] ^ self.z[k]) as usize]
    }
}
