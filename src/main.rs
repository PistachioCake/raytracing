use std::io::{BufWriter, Write};

use raytracing::ray::Ray;
use raytracing::units::{write_color, Color, Point, Vector};

fn ray_color(ray: &Ray) -> Color {
    let unit_direct = ray.direct.normalize();
    let a = (unit_direct.y + 1.) / 2.;
    Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // image
    let aspect_ratio: f32 = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio).floor() as i32;
    let image_height = image_height.max(1);

    // camera
    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * image_width as f32 / image_height as f32;

    let camera_center: Point = Point::ZERO;

    // vectors framing the viewport
    let viewport_u = Vector::new(viewport_width, 0., 0.);
    let viewport_v = Vector::new(0., -viewport_height, 0.);

    // vectors between the center of each pixel
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vector::new(0., 0., focal_length) - (viewport_u + viewport_v) / 2.;
    let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.;

    println!("P3\n{image_width} {image_height}\n255");

    let stdout = std::io::stdout().lock();
    let mut stdout = BufWriter::new(stdout);

    let mut stderr = std::io::stderr().lock();
    for i in 0..image_height {
        write!(stderr, "\rScanlines remaining: {:3}", image_height - i).unwrap();
        stderr.flush().unwrap();

        for j in 0..image_width {
            let origin = pixel_00_loc + pixel_delta_u * j as f32 + pixel_delta_v * i as f32;
            let direct = origin - camera_center;

            let ray = Ray { origin, direct };

            let color = ray_color(&ray);
            write_color(color, &mut stdout);
        }
    }
    stdout.flush().unwrap();
    write!(stderr, "\r{:30}\n", "Done.").unwrap();
}
