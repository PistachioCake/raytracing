use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::units::Point;

fn main() {
    // image
    let aspect_ratio: f32 = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio).floor() as i32;
    let image_height = image_height.max(1);

    // world
    let objects: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere {
            center: Point::new(0., 0., -1.),
            radius: 0.5,
        }),
        Rc::new(Sphere {
            center: Point::new(0., -100.5, -1.),
            radius: 100.,
        }),
    ];

    let world = HittableList { objects };

    // camera
    let camera = Camera::new(image_width, image_height);

    camera.render(&world);
}
