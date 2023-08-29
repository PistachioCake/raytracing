use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec3<T>([T; 3]);

impl<T: Add> Add for Vec3<T> {
    type Output = Vec3<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        // Vec3(self.0.zip(rhs.0).map(|(l, r)| T::add(l, r)))

        let [x, y, z] = self.0;
        let [a, b, c] = rhs.0;

        Vec3([x + a, y + b, z + c])
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        // self.0
        //     .iter_mut()
        //     .zip(rhs.0.into_iter())
        //     .map(|(l, r)| T::add_assign(l, r))
        //     .count();

        let [a, b, c] = rhs.0;
        self.0[0] += a;
        self.0[1] += b;
        self.0[2] += c;
    }
}

impl<T: Sub> Sub for Vec3<T> {
    type Output = Vec3<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        // Vec3(self.0.zip(rhs.0).map(|(l, r)| T::sub(l, r)))

        let [x, y, z] = self.0;
        let [a, b, c] = rhs.0;

        Vec3([x - a, y - b, z - c])
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        // self.0
        //     .iter_mut()
        //     .zip(rhs.0.into_iter())
        //     .map(|(l, r)| T::sub_assign(l, r))
        //     .count();

        let [a, b, c] = rhs.0;
        self.0[0] -= a;
        self.0[1] -= b;
        self.0[2] -= c;
    }
}

impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Vec3(self.0.map(T::neg))
    }
}

impl<T: Mul> Mul for Vec3<T> {
    type Output = Vec3<<T as Mul>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        let [x, y, z] = self.0;
        let [a, b, c] = rhs.0;

        Vec3([x * a, y * b, z * c])
    }
}

impl<T: Mul + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<<T as Mul>::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        let [x, y, z] = self.0;

        Vec3([x * rhs, y * rhs, z * rhs])
    }
}

impl<T: MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        // self.0
        //     .iter_mut()
        //     .zip(rhs.0.into_iter())
        //     .map(|(l, r)| T::mul_assign(l, r))
        //     .count();

        let [a, b, c] = rhs.0;
        self.0[0] *= a;
        self.0[1] *= b;
        self.0[2] *= c;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        // self.0
        //     .iter_mut()
        //     .map(|l| T::mul_assign(l, rhs))
        //     .count();

        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Vec3<T> {
    pub fn dot(self, rhs: Self) -> T {
        let [x, y, z] = self.0;
        let [a, b, c] = rhs.0;
        a * x + b * y + c * z
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vec3<T> {
    pub fn cross(self, rhs: Self) -> Self {
        let [x, y, z] = self.0;
        let [a, b, c] = rhs.0;
        Vec3([y * c - z * b, z * a - x * c, x * b - y * a])
    }
}

impl Vec3<f32> {
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self * (1. / self.length())
    }
}
