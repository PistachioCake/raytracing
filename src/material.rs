use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    units::{random_unit_vector, reflect, refract, Color},
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

pub struct Dielectric {
    pub ir: f32,
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
        let reflected = reflect(&ray.direct, &hit.normal);

        let scattered = Ray {
            origin: hit.p,
            direct: reflected + random_unit_vector() * self.fuzz,
        };

        Some((self.albedo, scattered))
    }
}

impl Dielectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::ONE;
        let refraction_ratio = if hit.front_face {
            self.ir.recip()
        } else {
            self.ir
        };

        let unit_direction = ray.direct.normalize();
        let cos_theta = hit.normal.dot(-unit_direction).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let cannot_refract =
            cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random();

        let direct = if cannot_refract {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };

        let scattered = Ray {
            origin: hit.p,
            direct,
        };

        Some((attenuation, scattered))
    }
}
