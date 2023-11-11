use std::alloc::Allocator;

use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    units::{random_unit_vector, reflect, refract, Color},
};

pub struct MatRecord {
    pub scatter: Option<(Color, Ray)>,
    pub emit: Option<Color>,
}

pub trait Material: Sync + Send {
    fn hit_info(&self, ray: &Ray, hit: &HitRecord) -> MatRecord;
}

pub struct Lambertian<'a> {
    pub albedo: &'a dyn Texture,
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

pub struct Dielectric {
    pub ir: f32,
}

pub struct DiffuseLight<'a> {
    pub emit: &'a dyn Texture,
}

impl<'a> Lambertian<'a> {
    pub fn new_with_color<A: Allocator + 'a>(color: Color, alloc: A) -> Self {
        let color = Box::leak(Box::new_in(SolidColor { color }, alloc));
        Self { albedo: color }
    }
}

impl Material for Lambertian<'_> {
    fn hit_info(&self, ray: &Ray, hit: &HitRecord) -> MatRecord {
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
            time: ray.time,
        };

        let color = self.albedo.value(hit.uv, hit.p);

        MatRecord {
            scatter: Some((color, scattered)),
            emit: None,
        }
    }
}

impl Material for Metal {
    fn hit_info(&self, ray: &Ray, hit: &HitRecord) -> MatRecord {
        let reflected = reflect(&ray.direct, &hit.normal);

        let scattered = Ray {
            origin: hit.p,
            direct: reflected + random_unit_vector() * self.fuzz,
            time: ray.time,
        };

        MatRecord {
            scatter: Some((self.albedo, scattered)),
            emit: None,
        }
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
    fn hit_info(&self, ray: &Ray, hit: &HitRecord) -> MatRecord {
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
            time: ray.time,
        };

        MatRecord {
            scatter: Some((attenuation, scattered)),
            emit: None,
        }
    }
}

impl<'a> DiffuseLight<'a> {
    pub fn new_with_color<A: Allocator + 'a>(color: Color, alloc: A) -> Self {
        let color = Box::leak(Box::new_in(SolidColor { color }, alloc));
        Self { emit: color }
    }
}

impl Material for DiffuseLight<'_> {
    fn hit_info(&self, _ray: &Ray, hit: &HitRecord) -> MatRecord {
        let color = self.emit.value(hit.uv, hit.p);

        MatRecord {
            scatter: None,
            emit: Some(color),
        }
    }
}
