use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    process::Output,
};

extern crate num_traits;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy + num_traits::Num> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn only(value: T) -> Self {
        Self::new(value, value, value)
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

macro_rules! impl_bin_op {
    ($t: ident :: $method: ident) => {
        impl<T: $t<Output = T>> $t for Vec3<T> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    x: $t::$method(self.x, rhs.x),
                    y: $t::$method(self.y, rhs.y),
                    z: $t::$method(self.z, rhs.z),
                }
            }
        }

        impl<T: $t<Output = T> + Copy> $t<T> for Vec3<T> {
            type Output = Vec3<T>;

            fn $method(self, rhs: T) -> Self::Output {
                Self::Output {
                    x: $t::$method(self.x, rhs),
                    y: $t::$method(self.y, rhs),
                    z: $t::$method(self.z, rhs),
                }
            }
        }
    };
}

macro_rules! impl_bin_assign_op {
    ($t: ident :: $method: ident) => {
        impl<T: $t> $t for Vec3<T> {
            fn $method(&mut self, rhs: Self) {
                $t::$method(&mut self.x, rhs.x);
                $t::$method(&mut self.y, rhs.y);
                $t::$method(&mut self.z, rhs.z);
            }
        }

        impl<T: $t + Copy> $t<T> for Vec3<T> {
            fn $method(&mut self, rhs: T) {
                $t::$method(&mut self.x, rhs);
                $t::$method(&mut self.y, rhs);
                $t::$method(&mut self.z, rhs);
            }
        }
    };
}

impl_bin_op!(Add::add);
impl_bin_op!(Sub::sub);
impl_bin_op!(Mul::mul);
impl_bin_op!(Div::div);
impl_bin_assign_op!(AddAssign::add_assign);
impl_bin_assign_op!(SubAssign::sub_assign);
impl_bin_assign_op!(MulAssign::mul_assign);
impl_bin_assign_op!(DivAssign::div_assign);

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.y,
            _ => panic!("Index {} out of 0 - 2 !", index),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index {} out of 0 - 2 !", index),
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: num_traits::Float> Vec3<T> {
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        self.mul(T::one() / self.length())
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub type Point3d = Vec3<f64>;
pub type Color = Vec3<f64>;
pub type Vec3d = Vec3<f64>;
