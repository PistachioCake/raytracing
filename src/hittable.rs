use crate::{
    ray::Ray,
    units::{Point, Vector},
};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point, outward_normal: Vector, t: f32) -> Self {
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
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direct.length_squared();
        let half_b = oc.dot(ray.direct);
        let c = oc.length_squared() - self.radius * self.radius;

        let discr = half_b * half_b - a * c;
        if discr < 0. {
            return None;
        }

        // find the nearest root that lies in the acceptable range
        let t = {
            let r1 = (-half_b - discr.sqrt()) / a;
            if tmin < r1 && r1 < tmax {
                r1
            } else {
                let r2 = (-half_b + discr.sqrt()) / a;
                if tmin < r2 && r2 < tmax {
                    r2
                } else {
                    return None;
                }
            }
        };

        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(ray, p, outward_normal, t))
    }
}
