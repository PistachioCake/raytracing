use bumpalo::Bump;
use rand::{random, thread_rng, Rng};

use raytracing::{
    camera::CameraBuilder,
    hittable::{bvh::BvhNode, Hittable, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    texture::{GlobalChecker, SolidColor},
    time_utils::{Linear, Unchanging},
    units::{Color, Point, Vector},
};

fn main() {
    let bump = Bump::new();
    let mut rng = thread_rng();

    // world
    let mut objects: Vec<&dyn Hittable> = Vec::with_capacity(124);

    let ground_material = bump.alloc(Lambertian {
        albedo: bump.alloc(GlobalChecker::new_colors(
            0.32,
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
            &bump,
        )),
    });

    objects.push(bump.alloc(Sphere::<Unchanging>::new(
        Point::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f32, b as f32);
            let choose_mat: f32 = rng.gen();
            let center = Point::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * random::<f32>());

            if (center - Point::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let material: &dyn Material = if choose_mat < 0.8 {
                let p1 = Color::new(rng.gen(), rng.gen(), rng.gen());
                let p2 = Color::new(rng.gen(), rng.gen(), rng.gen());
                let color = p1 * p2;
                bump.alloc(Lambertian {
                    albedo: bump.alloc(SolidColor { color }),
                })
            } else if choose_mat < 0.95 {
                let albedo = Color::new(
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                );
                let fuzz = rng.gen_range(0.0..0.5);
                bump.alloc(Metal { albedo, fuzz })
            } else {
                bump.alloc(Dielectric { ir: 1.5 })
            };

            if choose_mat < 0.8 {
                let velocity = Vector::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                objects.push(bump.alloc(Sphere::<Linear>::new(
                    (center, center + velocity),
                    0.2,
                    material,
                )))
            } else {
                objects.push(bump.alloc(Sphere::<Unchanging>::new(center, 0.2, material)))
            }
        }
    }

    objects.push(bump.alloc(Sphere::<Unchanging>::new(
        Point::new(0., 1., 0.),
        1.,
        bump.alloc(Dielectric { ir: 1.5 }),
    )));

    objects.push(bump.alloc(Sphere::<Unchanging>::new(
        Point::new(-4., 1., 0.),
        1.,
        bump.alloc(Lambertian {
            albedo: bump.alloc(SolidColor {
                color: Color::new(0.4, 0.2, 0.1),
            }),
        }),
    )));

    objects.push(bump.alloc(Sphere::<Unchanging>::new(
        Point::new(4., 1., 0.),
        1.,
        bump.alloc(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 1.,
        }),
    )));

    let world = BvhNode::new(objects, &bump);

    // camera
    let camera = CameraBuilder::default()
        .with_aspect_ratio(16. / 9.)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(20.)
        .with_lookfrom(Point::new(13., 2., 3.))
        .with_lookat(Point::new(0., 0., 0.))
        .with_vup(Vector::new(0., 1., 0.))
        .with_defocus_angle(0.6)
        .with_focus_dist(10.);
    let camera = camera.build();

    camera.render(&world);
}
