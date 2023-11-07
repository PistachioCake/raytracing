pub mod interval;
pub mod sphere;

use crate::{
    material::Material,
    ray::Ray,
    units::{Point, Vector},
};

pub use self::interval::Interval;
pub use self::sphere::Sphere;

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector,
    pub mat: &'a dyn Material,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> HitRecord<'a> {
    pub fn new(ray: &Ray, p: Point, outward_normal: Vector, mat: &'a dyn Material, t: f32) -> Self {
        // let p = ray.at(t);
        let front_face = ray.direct.dot(outward_normal) < 0.;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, mut ray_t: Interval<f32>) -> Option<HitRecord> {
        let mut hit_record = None;

        for object in &self.objects {
            let hit = object.hit(ray, ray_t);
            if let Some(ref rec) = hit {
                ray_t.max = rec.t;
                hit_record = hit;
            }
        }

        hit_record
    }
}
