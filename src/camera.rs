use std::io::{BufWriter, Write};

use crate::{
    hittable::{Hittable, Interval},
    ray::Ray,
    units::{write_color, Color, Point, Vector},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,

    center: Point,

    pixel_00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32) -> Self {
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

        Self {
            image_width,
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
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

        for i in 0..self.image_height {
            write!(stderr, "\rScanlines remaining: {:3}", self.image_height - i).unwrap();
            stderr.flush().unwrap();

            for j in 0..self.image_width {
                let pixel_center = self.pixel_00_loc
                    + self.pixel_delta_u * j as f32
                    + self.pixel_delta_v * i as f32;
                let direct = pixel_center - self.center;

                let ray = Ray {
                    origin: self.center,
                    direct,
                };

                let color = Camera::ray_color(&ray, world);
                write_color(color, &mut stdout);
            }
        }
        stdout.flush().unwrap();
        write!(stderr, "\r{:30}\n", "Done.").unwrap();
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        let hit = world.hit(ray, Interval::<f32>::POSITIVE);
        if let Some(hit) = hit {
            return ((hit.normal + Vector::new(1., 1., 1.)) / 2.).cast();
        }

        let unit_direct = ray.direct.normalize();
        let a = (unit_direct.y + 1.) / 2.;
        Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
