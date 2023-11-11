use crate::{
    ray::Ray,
    units::{Point, Vector, WorldSpace},
};

use super::{HitRecord, Hittable, Interval, AABB};

pub struct Translate<'a> {
    object: &'a dyn Hittable,
    offset: Vector,
    aabb: AABB<f32>,
}

pub struct Rotate<'a, const AXIS: usize = 1> {
    object: &'a dyn Hittable,
    sin: f32,
    cos: f32,
    aabb: AABB<f32>,
}

impl<'a> Translate<'a> {
    pub fn new(object: &'a dyn Hittable, offset: Vector) -> Self {
        let aabb = object.bounding_box().offset(offset);
        Self {
            object,
            offset,
            aabb,
        }
    }
}

impl<'a> Hittable for Translate<'a> {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord> {
        let new_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };

        self.object.hit(&new_ray, ray_t).map(|mut rec| {
            rec.p += self.offset;
            rec
        })
    }

    fn bounding_box(&self) -> AABB<f32> {
        self.aabb
    }
}

impl<'a, const AXIS: usize> Rotate<'a, AXIS> {
    pub fn new(object: &'a dyn Hittable, angle: f32) -> Self {
        let (sin, cos) = angle.to_radians().sin_cos();
        let aabb = AABB::EMPTY;

        let axis1 = (AXIS + 1) % 3;
        let axis2 = (AXIS + 2) % 3;

        for mut corner in object.bounding_box().to_corners::<WorldSpace>() {
            (corner[axis1], corner[axis2]) = (
                cos * corner[axis1] - sin * corner[axis2],
                sin * corner[axis1] + cos * corner[axis2],
            );
            aabb.insert(corner);
        }

        Self {
            object,
            sin,
            cos,
            aabb,
        }
    }

    fn rotate(&self, mut point: Point) -> Point {
        let axis1 = (AXIS + 1) % 3;
        let axis2 = (AXIS + 2) % 3;

        (point[axis1], point[axis2]) = (
            self.cos * point[axis1] - self.sin * point[axis2],
            self.sin * point[axis1] + self.cos * point[axis2],
        );

        point
    }

    fn unrotate(&self, mut point: Point) -> Point {
        let axis1 = (AXIS + 1) % 3;
        let axis2 = (AXIS + 2) % 3;

        (point[axis1], point[axis2]) = (
            self.cos * point[axis1] + self.sin * point[axis2],
            -self.sin * point[axis1] + self.cos * point[axis2],
        );

        point
    }
}

impl<'a, const AXIS: usize> Hittable for Rotate<'a, AXIS> {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord> {
        let new_ray = Ray {
            origin: self.unrotate(ray.origin),
            direct: self.unrotate(ray.direct.to_point()).to_vector(),
            ..*ray
        };

        self.object.hit(&new_ray, ray_t).map(|mut rec| {
            rec.p = self.rotate(rec.p);
            rec.normal = self.rotate(rec.normal.to_point()).to_vector();
            rec
        })
    }

    fn bounding_box(&self) -> AABB<f32> {
        self.aabb
    }
}
