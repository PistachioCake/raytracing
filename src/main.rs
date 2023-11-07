use bumpalo::Bump;
use rand::{random, thread_rng, Rng};

use raytracing::camera::CameraBuilder;
use raytracing::hittable::{HittableList, Sphere};
use raytracing::material::{Dielectric, Lambertian, Material, Metal};
use raytracing::time_utils::{Linear, Unchanging};
use raytracing::units::{Color, Point, Vector};

fn main() {
    let bump = Bump::new();
    let mut rng = thread_rng();

    // world
    let mut world = HittableList {
        objects: Vec::with_capacity(124),
    };

    let ground_material = &(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(bump.alloc(Sphere::<Unchanging> {
        center: Point::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    }));

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
                let albedo = p1 * p2;
                bump.alloc(Lambertian { albedo })
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
                world.add(bump.alloc(Sphere::<Linear> {
                    center: (center, center + velocity),
                    radius: 0.2,
                    material,
                }))
            } else {
                world.add(bump.alloc(Sphere::<Unchanging> {
                    center,
                    radius: 0.2,
                    material,
                }))
            }
        }
    }

    world.add(bump.alloc(Sphere::<Unchanging> {
        center: Point::new(0., 1., 0.),
        radius: 1.,
        material: bump.alloc(Dielectric { ir: 1.5 }),
    }));

    world.add(bump.alloc(Sphere::<Unchanging> {
        center: Point::new(-4., 1., 0.),
        radius: 1.,
        material: bump.alloc(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    }));

    world.add(bump.alloc(Sphere::<Unchanging> {
        center: Point::new(4., 1., 0.),
        radius: 1.,
        material: bump.alloc(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 1.,
        }),
    }));

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
