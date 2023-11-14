use std::{alloc::Allocator, path::Path};

use image::{io::Reader, RgbImage};

use crate::units::{Color, Point, TexCoord};

pub mod perlin;

pub trait Texture: Sync {
    fn value(&self, uv: TexCoord, point: Point) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _uv: TexCoord, _pointt: Point) -> Color {
        self.color
    }
}

pub struct GlobalChecker<'a> {
    pub inv_scale: f32,
    pub even: &'a dyn Texture,
    pub odd: &'a dyn Texture,
}

impl<'a> GlobalChecker<'a> {
    pub fn new(scale: f32, even: &'a dyn Texture, odd: &'a dyn Texture) -> Self {
        Self {
            inv_scale: scale.recip(),
            even,
            odd,
        }
    }

    pub fn new_colors<A: Allocator + Copy + 'a>(
        scale: f32,
        even: Color,
        odd: Color,
        alloc: A,
    ) -> Self {
        let even = Box::leak(Box::new_in(SolidColor { color: even }, alloc));
        let odd = Box::leak(Box::new_in(SolidColor { color: odd }, alloc));

        Self::new(scale, even, odd)
    }
}

impl Texture for GlobalChecker<'_> {
    fn value(&self, uv: TexCoord, point: Point) -> Color {
        let is_even = point
            .as_array()
            .iter()
            .map(|coord| (coord * self.inv_scale).floor() as i32)
            .sum::<i32>()
            % 2
            == 0;

        (if is_even { self.even } else { self.odd }).value(uv, point)
    }
}

pub struct ImageTexture {
    image: RgbImage,
}

impl Texture for ImageTexture {
    fn value(&self, uv: TexCoord, _point: Point) -> Color {
        if self.image.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = uv.x.clamp(0.0, 1.0);
        let v = 1.0 - uv.y.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f32).floor() as u32;
        let j = (v * self.image.height() as f32).floor() as u32;
        let pixel = self.image.get_pixel(i, j).0;

        Color::from_array(pixel.map(|channel| channel as f32 / 255.0))
    }
}

impl ImageTexture {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let image = Reader::open(path).ok()?.decode().ok()?.into_rgb8();

        Some(Self { image })
    }
}
