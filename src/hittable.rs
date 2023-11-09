pub mod aabb;
pub mod bvh;
pub mod interval;
pub mod sphere;

use crate::{
    material::Material,
    ray::Ray,
    units::{Point, TexCoord, Vector},
};

pub use self::aabb::AABB;
pub use self::interval::Interval;
pub use self::sphere::Sphere;

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector,
    pub mat: &'a dyn Material,
    pub t: f32,
    pub uv: TexCoord,
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB<f32>;
}

#[derive(Default)]
pub struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
    aabb: AABB<f32>,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        ray: &Ray,
        p: Point,
        outward_normal: Vector,
        uv: TexCoord,
        mat: &'a dyn Material,
        t: f32,
    ) -> Self {
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
            uv,
            mat,
        }
    }
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            aabb: AABB::EMPTY,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
            aabb: AABB::EMPTY,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.aabb = AABB::EMPTY;
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
        self.aabb = self.aabb.combine(object.bounding_box());
    }

    pub fn from_vec(objects: Vec<&'a dyn Hittable>) -> Self {
        let mut aabb = AABB::EMPTY;
        for hittable in &objects {
            aabb = aabb.combine(hittable.bounding_box());
        }

        Self { objects, aabb }
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

    fn bounding_box(&self) -> AABB<f32> {
        self.aabb
    }
}
