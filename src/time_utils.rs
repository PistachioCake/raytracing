use std::ops::{Add, Mul, Sub};

pub trait Movement<T> {
    type Storage: Send + Sync;

    fn get_at_time(this: &Self::Storage, time: f32) -> T;
}

pub struct Unchanging;

impl<T: Copy + Send + Sync> Movement<T> for Unchanging {
    type Storage = T;

    fn get_at_time(this: &Self::Storage, _time: f32) -> T {
        *this
    }
}

pub struct Linear;

impl<U: Mul<f32, Output = U>, T: Add<U, Output = T> + Sub<Output = U> + Copy + Send + Sync>
    Movement<T> for Linear
{
    type Storage = (T, T);

    fn get_at_time((a, b): &Self::Storage, time: f32) -> T {
        *a + (*b - *a) * time
    }
}
