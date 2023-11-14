use crate::{hittable::AABB, units::Point};

pub trait Movement<T> {
    type Storage: Sync;

    fn get_at_time(this: &Self::Storage, time: f32) -> T;
    fn bounding_box(this: &Self::Storage) -> AABB<f32>;
}

pub struct Unchanging;

impl Movement<Point> for Unchanging {
    type Storage = Point;

    fn get_at_time(this: &Self::Storage, _time: f32) -> Point {
        *this
    }

    fn bounding_box(this: &Self::Storage) -> AABB<f32> {
        AABB::<f32>::from_corners(*this, *this)
    }
}

pub struct Linear;

impl Movement<Point> for Linear {
    type Storage = (Point, Point);

    fn get_at_time((a, b): &Self::Storage, time: f32) -> Point {
        *a + (*b - *a) * time
    }

    fn bounding_box((a, b): &Self::Storage) -> AABB<f32> {
        AABB::<f32>::from_corners(*a, *b)
    }
}
