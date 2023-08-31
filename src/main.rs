use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::material::Lambertian;
use raytracing::units::{Color, Point};

fn main() {
    // image
    let aspect_ratio: f32 = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio).floor() as i32;
    let image_height = image_height.max(1);

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
    let camera = Camera::new(image_width, image_height, Some(100), Some(50), 90.);

    camera.render(&world);
}
