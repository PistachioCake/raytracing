use crate::{
    hittable::HitRecord,
    ray::Ray,
    units::{random_unit_vector, Color},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let direct = {
            let direct = hit.normal + random_unit_vector();
            if direct.max_element() < f32::EPSILON {
                hit.normal
            } else {
                direct
            }
        };

        let scattered = Ray {
            origin: hit.p,
            direct,
        };

        Some((self.albedo, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direct - hit.normal * 2. * ray.direct.dot(hit.normal);

        let scattered = Ray {
            origin: hit.p,
            direct: reflected + random_unit_vector() * self.fuzz,
        };

        Some((self.albedo, scattered))
    }
}
