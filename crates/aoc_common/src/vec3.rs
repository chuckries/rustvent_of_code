use std::iter::Sum;

use num_traits::{PrimInt, Signed};

use crate::Vec2;

pub type Vec3us = Vec3<usize>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3u64 = Vec3<u64>;

pub type Selector<T> = fn(&Vec3<T>) -> T;
pub type SelectorMut<T> = fn(&mut Vec3<T>) -> &mut T;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Default, Debug)]
pub struct Vec3<T: PrimInt> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: PrimInt> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z}
    }

    pub const fn x(&self) -> T {
        self.x
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub const fn y(&self) -> T {
        self.y
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub const fn z(&self) -> T {
        self.z
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.z
    }

    pub const fn xy(&self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub const fn xz(&self) -> Vec2<T> {
        Vec2::new(self.x, self.z)
    }

    pub const fn yz(&self) -> Vec2<T> {
        Vec2::new(self.y, self.z)
    }

    pub fn zero() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::zero() }
    }

    pub fn one() -> Self {
        Self { x: T::one(), y: T::one(), z: T::one() }
    }

    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero(), z: T::zero() }
    }

    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one(), z: T::zero() }
    }

    pub fn unit_z() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::one() }
    }

    pub fn adjacent(&self) -> impl Iterator<Item = Self> {
        [
            Self { x: self.x           , y: self.y           , z: self.z - T::one() },
            Self { x: self.x           , y: self.y           , z: self.z + T::one() },
            Self { x: self.x           , y: self.y - T::one(), z: self.z            },
            Self { x: self.x           , y: self.y + T::one(), z: self.z            },
            Self { x: self.x - T::one(), y: self.y           , z: self.z            },
            Self { x: self.x + T::one(), y: self.y           , z: self.z            },
        ].into_iter()
    }
}

impl<T: PrimInt + Signed> Vec3<T> {
    pub fn cross(&self, b: Self) -> Self {
        let a = self;
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

macro_rules! manhattan_unsigned {
    ($unsigned:ty) => {
        impl Vec3<$unsigned> {
            pub fn manhattan(&self) -> $unsigned {
                self.x + self.y + self.z
            }

            pub fn manhattan_from(&self, other: Self) -> $unsigned {
                <$unsigned>::abs_diff(self.x, other.x) + <$unsigned>::abs_diff(self.y, other.y) + <$unsigned>::abs_diff(self.z, other.z)
            }
        }
    };
}

macro_rules! manhattan_signed {
    ($signed:ty) => {
        impl Vec3<$signed> {
            pub fn manhattan(&self) -> $signed {
                <$signed>::abs(self.x) + <$signed>::abs(self.y) + <$signed>::abs(self.z)
            }

            pub fn manhattan_from(&self, other: Self) -> $signed {
                <$signed>::abs(self.x - other.x) + <$signed>::abs(self.y - other.y) + <$signed>::abs(self.z - other.z)
            }
        }
    };
}

manhattan_unsigned!(usize);
manhattan_unsigned!(u32);
manhattan_unsigned!(u64);
manhattan_signed!(i32);
manhattan_signed!(i64);

impl <T: PrimInt + std::fmt::Display> std::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: PrimInt + std::ops::Neg<Output = T>> std::ops::Neg for Vec3<T> {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: PrimInt> std::ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self. z + rhs.z)
    }
}

impl<T: PrimInt> std::ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<T: PrimInt> std::ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self. z - rhs.z)
    }
}

impl<T: PrimInt> std::ops::SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<T:PrimInt> Sum for Vec3<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
    }
}

impl<T: PrimInt> From<(T, T, T)> for Vec3<T> {
    fn from(v: (T, T, T)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

impl<T: PrimInt> From<Vec3<T>> for (T, T, T) {
    fn from(v: Vec3<T>) -> Self {
        (v.x, v.y, v.z)
    }
}