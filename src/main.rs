use std::rc::Rc;

use raytracing::camera::{Camera, CameraBuilder};
use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::material::Lambertian;
use raytracing::units::{Color, Point};

fn main() {
    // world
    let r = std::f32::consts::FRAC_PI_4.cos();

    let objects: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere {
            center: Point::new(-r, 0., -1.),
            radius: r,
            material: Rc::new(Lambertian {
                albedo: Color::new(0., 0., 1.),
            }),
        }),
        Rc::new(Sphere {
            center: Point::new(r, 0., -1.),
            radius: r,
            material: Rc::new(Lambertian {
                albedo: Color::new(1., 0., 0.),
            }),
        }),
    ];

    let world = HittableList { objects };

    // camera
    let camera = CameraBuilder::default()
        .with_aspect_ratio(16. / 9.)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(90.);
    let camera = camera.build();

    camera.render(&world);
}
