use std::ops::{Index, IndexMut};

use glamour::{Point3, Unit};

use crate::ray::Ray;

use super::Interval;

#[derive(Clone, Copy)]
pub struct AABB<T> {
    pub x: Interval<T>,
    pub y: Interval<T>,
    pub z: Interval<T>,
}

impl<T> AABB<T> {
    pub fn new(x: Interval<T>, y: Interval<T>, z: Interval<T>) -> Self {
        Self { x, y, z }
    }

    pub fn from_corners<U: Unit>(a: Point3<U>, b: Point3<U>) -> AABB<U::Scalar> {
        let lo = a.min(b);
        let hi = a.max(b);

        AABB {
            x: Interval {
                min: lo.x,
                max: hi.x,
            },
            y: Interval {
                min: lo.y,
                max: hi.y,
            },
            z: Interval {
                min: lo.z,
                max: hi.z,
            },
        }
    }

    pub fn as_array(&self) -> [&Interval<T>; 3] {
        [&self.x, &self.y, &self.z]
    }
    pub fn as_array_mut(&mut self) -> [&mut Interval<T>; 3] {
        [&mut self.x, &mut self.y, &mut self.z]
    }
}

impl AABB<f32> {
    pub const EMPTY: Self = Self {
        x: Interval::<f32>::EMPTY,
        y: Interval::<f32>::EMPTY,
        z: Interval::<f32>::EMPTY,
    };
}

impl Default for AABB<f32> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T> Index<usize> for AABB<T> {
    type Output = Interval<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_array()[index]
    }
}

impl<T> IndexMut<usize> for AABB<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut()[index]
    }
}

// we could add an f64 impl here, but there's no need

impl AABB<f32> {
    pub fn hit(&self, ray: &Ray, mut ray_t: Interval<f32>) -> bool {
        for axis in 0..3 {
            let inv_d = ray.direct[axis].recip();
            let orig = ray.origin[axis];

            let (t0, t1) = (
                (self[axis].min - orig) * inv_d,
                (self[axis].max - orig) * inv_d,
            );

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn expand(self, delta: f32) -> Self {
        Self {
            x: self.x.expand(delta),
            y: self.y.expand(delta),
            z: self.z.expand(delta),
        }
    }

    pub fn combine(self, other: Self) -> Self {
        Self {
            x: self.x.combine(other.x),
            y: self.y.combine(other.y),
            z: self.z.combine(other.z),
        }
    }
}
