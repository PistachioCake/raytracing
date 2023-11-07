#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}
impl<T: PartialOrd> Interval<T> {
    pub fn contains(self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: T) -> bool {
        self.min < x && x < self.max
    }
}

impl<T: Ord> Interval<T> {
    // cannot implement because of compiler error
    // "upstream crates may add a new impl of trait `std::cmp::Ord` for type `f32`/`f64`"
    //
    // pub fn clamp(self, x: T) -> T {
    //     x.clamp(self.min, self.max)
    // }
}

impl Interval<f32> {
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    pub const POSITIVE: Self = Self {
        min: 0.001,
        max: f32::INFINITY,
    };

    pub fn clamp(self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}

impl Interval<f64> {
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub const POSITIVE: Self = Self {
        min: 0.001,
        max: f64::INFINITY,
    };

    pub fn clamp(self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
