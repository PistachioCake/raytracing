use std::io::{BufWriter, Write};

use indicatif::ParallelProgressIterator;
use rand::random;
use rayon::prelude::*;

use crate::{
    hittable::{Hittable, Interval},
    ray::Ray,
    units::{random_in_unit_disk, write_color, Color, Point, Vector},
};

pub struct CameraBuilder {
    aspect_ratio: f32,
    image_width: Option<i32>,
    image_height: Option<i32>,

    samples_per_pixel: i32,
    max_depth: i32,

    vfov: f32,
    lookfrom: Point,
    lookat: Point,
    vup: Vector,

    defocus_angle: f32,
    focus_dist: f32,
}

pub struct Camera {
    image_width: i32,
    image_height: i32,

    center: Point,

    pixel_00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,

    defocus_angle: f32,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,

    samples_per_pixel: i32,
    max_depth: i32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.,
            image_width: Some(100),
            image_height: None,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 1.,
            lookfrom: Point::new(0., 0., -1.),
            lookat: Point::new(0., 0., 0.),
            vup: Vector::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_dist: 10.,
        }
    }
}

impl CameraBuilder {
    pub fn with_aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_image_width(&mut self, image_width: i32) -> &mut Self {
        self.image_width = Some(image_width);
        self
    }

    pub fn with_image_height(&mut self, image_height: i32) -> &mut Self {
        self.image_height = Some(image_height);
        self
    }

    pub fn with_samples_per_pixel(&mut self, samples_per_pixel: i32) -> &mut Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_max_depth(&mut self, max_depth: i32) -> &mut Self {
        self.max_depth = max_depth;
        self
    }

    pub fn with_vfov(&mut self, vfov: f32) -> &mut Self {
        self.vfov = vfov;
        self
    }

    pub fn with_lookfrom(&mut self, lookfrom: Point) -> &mut Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn with_lookat(&mut self, lookat: Point) -> &mut Self {
        self.lookat = lookat;
        self
    }

    pub fn with_vup(&mut self, vup: Vector) -> &mut Self {
        self.vup = vup;
        self
    }

    pub fn with_defocus_angle(&mut self, defocus_angle: f32) -> &mut Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn with_focus_dist(&mut self, focus_dist: f32) -> &mut Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn build(self) -> Camera {
        let CameraBuilder {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        } = self;

        let (image_width, image_height) = match (image_width, image_height) {
            (None, None) => (100, (100. / aspect_ratio).floor() as _),
            (Some(image_width), None) => (
                image_width,
                (image_width as f32 / aspect_ratio).floor() as _,
            ),
            (None, Some(image_height)) => (
                (image_height as f32 * aspect_ratio).floor() as _,
                image_height,
            ),
            (Some(image_width), Some(image_height)) => (image_width, image_height),
        };

        let center: Point = lookfrom;

        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = focus_dist * 2. * h;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        // basis vectors for the camera coordinate system
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        assert!(v.is_normalized());

        // vectors framing the viewport
        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        // vectors between the center of each pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // location of the upper left pixel
        let viewport_upper_left = center - (w * focus_dist) - (viewport_u + viewport_v) / 2.;
        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.;

        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            samples_per_pixel,
            max_depth,
        }
    }
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) {
        let image: Vec<_> = (0..self.image_height)
            .into_par_iter()
            .progress()
            .map(|j| {
                let scanline: Vec<_> = (0..self.image_width)
                    .map(|i| {
                        let mut color = Color::ZERO;
                        for _ in 0..self.samples_per_pixel {
                            let ray = self.get_ray(i, j);

                            color += Camera::ray_color(&ray, world, self.max_depth);
                        }

                        color /= self.samples_per_pixel as f32;

                        // write_color(&mut stdout, color);
                        color
                    })
                    .collect();
                scanline
            })
            .collect();
        let stdout = std::io::stdout().lock();
        let mut stdout = BufWriter::new(stdout);
        let mut stderr = std::io::stderr().lock();

        write!(
            stdout,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();

        for line in image {
            for col in line {
                write_color(&mut stdout, col);
            }
        }

        stdout.flush().unwrap();
        write!(stderr, "\r{:30}\n", "Done.").unwrap();
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel_00_loc + self.pixel_delta_u * i as f32 + self.pixel_delta_v * j as f32;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direct = pixel_sample - origin;
        let time = random();

        Ray {
            origin,
            direct,
            time,
        }
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }

        let hit = world.hit(ray, Interval::<f32>::POSITIVE);
        if let Some(hit) = hit {
            if let Some((attenuation, scattered)) = hit.mat.scatter(ray, &hit) {
                return Self::ray_color(&scattered, world, depth - 1) * attenuation;
            }
            return Color::ZERO;
        }

        let unit_direct = ray.direct.normalize();
        let a = (unit_direct.y + 1.) / 2.;
        Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn pixel_sample_square(&self) -> Vector {
        let px = random::<f32>() - 0.5;
        let py = random::<f32>() - 0.5;

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
}
