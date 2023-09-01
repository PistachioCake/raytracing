use std::rc::Rc;

use raytracing::camera::CameraBuilder;
use raytracing::hittable::{Hittable, HittableList, Sphere};
use raytracing::material::{Dielectric, Lambertian, Metal};
use raytracing::units::{Color, Point, Vector};

fn main() {
    // world
    let material_left = Rc::new(Dielectric { ir: 1.5 });

    let objects: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere {
            center: Point::new(0., -100.5, -1.),
            radius: 100.,
            material: Rc::new(Lambertian {
                albedo: Color::new(0.8, 0.8, 0.),
            }),
        }),
        Rc::new(Sphere {
            center: Point::new(0., 0., -1.),
            radius: 0.5,
            material: Rc::new(Lambertian {
                albedo: Color::new(0.1, 0.2, 0.5),
            }),
        }),
        Rc::new(Sphere {
            center: Point::new(-1., 0., -1.),
            radius: 0.5,
            material: material_left.clone(),
        }),
        Rc::new(Sphere {
            center: Point::new(-1., 0., -1.),
            radius: -0.4,
            material: material_left.clone(),
        }),
        Rc::new(Sphere {
            center: Point::new(1., 0., -1.),
            radius: 0.5,
            material: Rc::new(Metal {
                albedo: Color::new(0.8, 0.6, 0.2),
                fuzz: 0.,
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
        .with_vfov(20.)
        .with_lookfrom(Point::new(-2., 2., 1.))
        .with_lookat(Point::new(0., 0., -1.))
        .with_vup(Vector::new(0., 1., 0.))
        .with_defocus_angle(10.)
        .with_focus_dist(3.4);
    let camera = camera.build();

    camera.render(&world);
}
