use std::sync::Arc;

use crate::{
    material::Material,
    ray::Ray,
    units::{Point, Vector},
};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub mat: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        p: Point,
        outward_normal: Vector,
        mat: Arc<dyn Material>,
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
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direct.length_squared();
        let half_b = oc.dot(ray.direct);
        let c = oc.length_squared() - self.radius * self.radius;

        let discr = half_b * half_b - a * c;
        if discr < 0. {
            return None;
        }

        // find the nearest root that lies in the acceptable range
        let t = [(-half_b - discr.sqrt()) / a, (-half_b + discr.sqrt()) / a]
            .into_iter()
            .find(|&root| ray_t.surrounds(root));
        let t = match t {
            Some(t) => t,
            None => return None,
        };

        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(
            ray,
            p,
            outward_normal,
            self.material.clone(),
            t,
        ))
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
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

impl<T: PartialOrd> Interval<T> {
    pub fn contains(self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: T) -> bool {
        self.min < x && x < self.max
    }
}

impl<T: Ord> Interval<T> {
    // cannot implement because of compiler error
    // "upstream crates may add a new impl of trait `std::cmp::Ord` for type `f32`/`f64`"
    //
    // pub fn clamp(self, x: T) -> T {
    //     x.clamp(self.min, self.max)
    // }
}

impl Interval<f32> {
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    pub const POSITIVE: Self = Self {
        min: 0.001,
        max: f32::INFINITY,
    };

    pub fn clamp(self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}

impl Interval<f64> {
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub const POSITIVE: Self = Self {
        min: 0.001,
        max: f64::INFINITY,
    };

    pub fn clamp(self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
