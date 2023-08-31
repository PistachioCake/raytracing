use crate::{hittable::HitRecord, ray::Ray, units::Color};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
