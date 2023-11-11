use glamour::Point2;

use crate::{
    material::Material,
    ray::Ray,
    units::{Point, Vector},
};

use super::{HitRecord, Hittable, Interval, AABB};

pub struct Quad<'a> {
    pub q: Point,
    pub u: Vector,
    pub v: Vector,

    pub normal: Vector,
    pub d: f32,
    pub w: Vector,

    pub material: &'a dyn Material,
    pub aabb: AABB<f32>,
}

impl<'a> Quad<'a> {
    pub fn new(q: Point, u: Vector, v: Vector, material: &'a dyn Material) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q.to_vector());
        let w = n / n.dot(n);

        let aabb = AABB::<f32>::from_corners(q, q + u + v).pad();

        Self {
            q,
            u,
            v,
            normal,
            d,
            w,
            material,
            aabb,
        }
    }

    pub fn new_box(a: Point, b: Point, material: &'a dyn Material) -> [Self; 6] {
        let (min, max) = (a.min(b), a.max(b));
        let Vector {
            x: dx,
            y: dy,
            z: dz,
        } = max - min;
        let edges = [
            Vector::new(dx, 0.0, 0.0),
            Vector::new(0.0, dy, 0.0),
            Vector::new(0.0, 0.0, dz),
        ];

        [
            (0, false),
            (0, true),
            (1, false),
            (1, true),
            (2, false),
            (2, true),
        ]
        .map(|(axis, top)| {
            let mut vert = min;
            let (u, v);
            if top {
                vert[axis] = max[axis];
                u = edges[(axis + 1) % 3];
                v = edges[(axis + 2) % 3];
            } else {
                u = edges[(axis + 2) % 3];
                v = edges[(axis + 1) % 3];
            }

            Quad::new(vert, u, v, material)
        })
    }
}

impl<'a> Hittable for Quad<'a> {
    fn hit(&self, ray: &Ray, ray_t: Interval<f32>) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direct);

        if denom.abs() < f32::EPSILON {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin.to_vector())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let p = ray.at(t);
        let planar_hit = p - self.q;
        let alpha = self.w.dot(planar_hit.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit));

        if !(0.0..1.0).contains(&alpha) || !(0.0..1.0).contains(&beta) {
            return None;
        }

        Some(HitRecord::new(
            ray,
            p,
            self.normal,
            Point2::new(alpha, beta),
            self.material,
            t,
        ))
    }

    fn bounding_box(&self) -> AABB<f32> {
        self.aabb
    }
}
