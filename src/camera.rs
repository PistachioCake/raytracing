use std::io::{BufWriter, Write};

use rand::random;

use crate::{
    hittable::{Hittable, Interval},
    ray::Ray,
    units::{random_on_hemisphere, write_color, Color, Point, Vector},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,

    center: Point,

    pixel_00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,

    samples_per_pixel: i32,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32, samples_per_pixel: Option<i32>) -> Self {
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        let center: Point = Point::ZERO;

        // vectors framing the viewport
        let viewport_u = Vector::new(viewport_width, 0., 0.);
        let viewport_v = Vector::new(0., -viewport_height, 0.);

        // vectors between the center of each pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // location of the upper left pixel
        let viewport_upper_left =
            center - Vector::new(0., 0., focal_length) - (viewport_u + viewport_v) / 2.;
        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.;

        let samples_per_pixel = samples_per_pixel.unwrap_or(10);

        Self {
            image_width,
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let stdout = std::io::stdout().lock();
        let mut stdout = BufWriter::new(stdout);
        let mut stderr = std::io::stderr().lock();

        write!(
            stdout,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();

        for j in 0..self.image_height {
            write!(stderr, "\rScanlines remaining: {:3}", self.image_height - j).unwrap();
            stderr.flush().unwrap();

            for i in 0..self.image_width {
                let mut color = Color::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);

                    color += Camera::ray_color(&ray, world);
                }

                color /= self.samples_per_pixel as f32;

                write_color(&mut stdout, color);
            }
        }
        stdout.flush().unwrap();
        write!(stderr, "\r{:30}\n", "Done.").unwrap();
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel_00_loc + self.pixel_delta_u * i as f32 + self.pixel_delta_v * j as f32;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let direct = pixel_sample - self.center;

        Ray {
            origin: self.center,
            direct,
        }
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        let hit = world.hit(ray, Interval::<f32>::POSITIVE);
        if let Some(hit) = hit {
            let bounce_direction = random_on_hemisphere(&hit.normal);
            return Self::ray_color(
                &Ray {
                    origin: hit.p,
                    direct: bounce_direction,
                },
                world,
            ) / 2.;
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
