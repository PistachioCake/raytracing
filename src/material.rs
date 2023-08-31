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

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::ONE;
        let refraction_ratio = if hit.front_face {
            self.ir.recip()
        } else {
            self.ir
        };

        let unit_direction = ray.direct.normalize();
        let refracted = refract(&unit_direction, &hit.normal, refraction_ratio);

        let scattered = Ray {
            origin: hit.p,
            direct: refracted,
        };

        Some((attenuation, scattered))
    }
}
