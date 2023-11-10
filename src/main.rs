#![feature(allocator_api)]

use std::alloc::Global;

use bumpalo::Bump;
use rand::{thread_rng, Rng};

use raytracing::{
    camera::CameraBuilder,
    hittable::{bvh::BvhNode, Hittable, HittableList, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    texture::{perlin::NoiseTexture, GlobalChecker, ImageTexture, SolidColor},
    time_utils::{Linear, Unchanging},
    units::{Color, Point, Vector},
};

fn main() {
    let world = match 4 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        _ => unimplemented!(),
    };

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

    camera.render(world);
}

fn two_perlin_spheres() -> &'static dyn Hittable {
    fn leak<T>(x: T) -> &'static T {
        Box::leak(Box::new(x))
    }

    let mut world = HittableList::with_capacity(2);

    let perlin = leak(NoiseTexture::new(4.0));
    let material = leak(Lambertian { albedo: perlin });

    world.add(leak(Sphere::<Unchanging>::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material,
    )));

    world.add(leak(Sphere::<Unchanging>::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    leak(world)
}

fn earth() -> &'static dyn Hittable {
    fn leak<T>(x: T) -> &'static T {
        Box::leak(Box::new(x))
    }

    let earth_texture = leak(ImageTexture::from_path("images/earthmap.jpg").unwrap());
    let earth_surface = leak(Lambertian {
        albedo: earth_texture,
    });
    let globe = leak(Sphere::<Unchanging>::new(Point::ZERO, 2.0, earth_surface));

    globe
}

fn two_spheres() -> &'static dyn Hittable {
    fn leak<T>(x: T) -> &'static T {
        Box::leak(Box::new(x))
    }

    let mut world = HittableList::with_capacity(2);

    let checker = leak(GlobalChecker::new_colors(
        0.8,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
        Global::default(),
    ));

    let material = leak(Lambertian { albedo: checker });

    world.add(leak(Sphere::<Unchanging>::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        material,
    )));

    world.add(leak(Sphere::<Unchanging>::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        material,
    )));

    leak(world)
}

fn random_spheres() -> &'static dyn Hittable {
    let bump: &_ = Box::leak(Box::new(Bump::new()));
    let mut rng = thread_rng();

    let mut objects: Vec<&dyn Hittable> = Vec::with_capacity(124);
    let ground_material = bump.alloc(Lambertian {
        albedo: bump.alloc(GlobalChecker::new_colors(
            0.32,
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
            bump,
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
            let center = Point::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());

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

    let world = BvhNode::new(objects, bump);

    bump.alloc(world)
}
