use crate::{material::Material, ray::Ray, time_utils::Movement, units::Point};

use super::{HitRecord, Hittable, Interval, AABB};

pub struct Sphere<'a, Center: Movement<Point>> {
    pub center: Center::Storage,
    pub radius: f32,
    pub material: &'a dyn Material,
    pub aabb: AABB<f32>,
}

impl<'a, Center: Movement<Point>> Sphere<'a, Center> {
    pub fn new(center: Center::Storage, radius: f32, material: &'a dyn Material) -> Self {
        let aabb = <Center as Movement<Point>>::bounding_box(&center).expand(radius * 2.0);

        Sphere {
            center,
            radius,
            material,
            aabb,
        }
    }
}

impl<Center: Movement<Point>> Hittable for Sphere<'_, Center> {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord> {
        let center = <Center as Movement<_>>::get_at_time(&self.center, ray.time);

        sphere_hit(center, self.radius, self.material, ray, ray_t)
    }

    fn bounding_box(&self) -> AABB<f32> {
        self.aabb
    }
}

fn sphere_hit<'a>(
    center: Point,
    radius: f32,
    material: &'a dyn Material,
    ray: &Ray,
    ray_t: Interval<f32>,
) -> Option<HitRecord<'a>> {
    let oc = ray.origin - center;
    let a = ray.direct.length_squared();
    let half_b = oc.dot(ray.direct);
    let c = oc.length_squared() - radius * radius;
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
    let outward_normal = (p - center) / radius;

    Some(HitRecord::new(ray, p, outward_normal, material, t))
}
