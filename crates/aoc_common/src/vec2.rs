use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div};

use num_traits::{PrimInt};

pub type Vec2us = Vec2<usize>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Vec2<T: PrimInt> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn zero() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }

    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero() }
    }

    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one() }
    }

    pub fn is_in_bounds(&self, bounds: Self) -> bool {
        self.x >= T::zero() && self.x < bounds.x && self.y >= T::zero() && self.y < bounds.y
    }

    pub fn adjacent(&self) -> impl Iterator<Item = Self> {
        [
            Self { x: self.x - T::one(), y: self.y            },
            Self { x: self.x + T::one(), y: self.y            },
            Self { x: self.x           , y: self.y - T::one() },
            Self { x: self.x           , y: self.y + T::one() },
        ].into_iter()
    }

    pub fn adjacent_non_negative(&self) -> impl Iterator<Item = Self> {
        let mut adj = Vec::with_capacity(4);

        if self.x > T::zero() { adj.push(Self { x: self.x - T::one(), y: self.y            }) }
                                adj.push(Self { x: self.x + T::one(), y: self.y            });
        if self.y > T::zero() { adj.push(Self { x: self.x           , y: self.y - T::one() }) }
                                adj.push(Self { x: self.x           , y: self.y + T::one() });

        adj.into_iter()
    }

    pub fn adjacent_bounded(&self, bounds: &Self) -> impl Iterator<Item = Self> {
        let mut adj = Vec::with_capacity(4);

        if self.x > T::zero()               { adj.push(Self { x: self.x - T::one(), y: self.y            }) }
        if self.x < bounds.x - T::one()     { adj.push(Self { x: self.x + T::one(), y: self.y            }) }
        if self.y > T::zero()               { adj.push(Self { x: self.x           , y: self.y - T::one() }) }
        if self.y < bounds.y - T::one()     { adj.push(Self { x: self.x           , y: self.y + T::one() }) }

        adj.into_iter()
    }

    pub fn surrouding_bounded(&self, bounds: &Self) -> impl Iterator<Item = Self> {
        let mut sur = Vec::with_capacity(8);
    
        if self.x > T::zero() && self.y > T::zero()                         { sur.push(Vec2 { x: self.x - T::one(), y: self.y - T::one() }); }
        if self.x > T::zero()                                               { sur.push(Vec2 { x: self.x - T::one(), y: self.y            }); }
        if self.x > T::zero() && self.y < bounds.y - T::one()               { sur.push(Vec2 { x: self.x - T::one(), y: self.y + T::one() }); }
        if self.y > T::zero()                                               { sur.push(Vec2 { x: self.x           , y: self.y - T::one() }); }
        if self.y < bounds.y - T::one()                                     { sur.push(Vec2 { x: self.x           , y: self.y + T::one() }); }
        if self.x < bounds.x - T::one() && self.y > T::zero()               { sur.push(Vec2 { x: self.x + T::one(), y: self.y - T::one() }); }
        if self.x < bounds.x - T::one()                                     { sur.push(Vec2 { x: self.x + T::one(), y: self.y            }); }
        if self.x < bounds.x - T::one() && self.y < bounds.y - T::one()     { sur.push(Vec2 { x: self.x + T::one(), y: self.y + T::one() }); }
        sur.into_iter()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(Self::new(T::zero(), T::zero()), *self)
    }
}

macro_rules! manhattan_unsigned {
    ($unsigned:ty) => {
        impl Vec2<$unsigned> {
            pub fn manhattan(&self) -> $unsigned {
                self.x + self.y
            }

            pub fn manhattan_from(&self, other: Self) -> $unsigned {
                <$unsigned>::abs_diff(self.x, other.x) + <$unsigned>::abs_diff(self.y, other.y)
            }
        }
    };
}

macro_rules! manhattan_signed {
    ($signed:ty) => {
        impl Vec2<$signed> {
            pub fn manhattan(&self) -> $signed {
                <$signed>::abs(self.x) + <$signed>::abs(self.y)
            }

            pub fn manhattan_from(&self, other: Self) -> $signed {
                <$signed>::abs(self.x - other.x) + <$signed>::abs(self.y - other.y)
            }
        }
    };
}

manhattan_unsigned!(usize);
manhattan_unsigned!(u32);
manhattan_unsigned!(u64);
manhattan_signed!(i32);
manhattan_signed!(i64);

impl <T: PrimInt + std::fmt::Display> std::fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl <T: PrimInt + Neg<Output = T>> Vec2<T> {
    pub fn rotate_left(&self) -> Self {
        Self { x: self.y, y: -self.x }
    }

    pub fn rotate_right(&self) -> Self {
        Self { x: -self.y, y: self. x }
    }
}

impl<T: PrimInt + Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl <T: PrimInt> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: PrimInt> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl <T: PrimInt> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl <T: PrimInt> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl <T: PrimInt> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl <T: PrimInt> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl <T: PrimInt> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl From<Vec2<i64>> for Vec2<usize> {
    fn from(v: Vec2<i64>) -> Self {
        Self { x: v.x as usize, y: v.y as usize }
    }
}

impl From<Vec2<usize>> for Vec2<i64> {
    fn from(v: Vec2<usize>) -> Self {
        Self { x: v.x as i64, y: v.y as i64 }
    }
}

impl<T: PrimInt> From<(T, T)> for Vec2<T> {
    fn from(v: (T, T)) -> Self {
        Vec2 { x: v.0, y: v.1 }
    }
}

impl<T: PrimInt> From<Vec2<T>> for (T, T) {
    fn from(v: Vec2<T>) -> Self {
        (v.x, v.y)
    }
}

pub struct Iter<T: PrimInt> {
    start: Vec2<T>,
    end: Vec2<T>,
    current: Vec2<T>,
    finished: bool
}

impl<T: PrimInt> Iter<T> {
    fn new(start: Vec2<T>, end: Vec2<T>) -> Self {

        Iter {
            start,
            end,
            current: start,
            finished: false
        }
    }
}

impl<T: PrimInt> Iterator for Iter<T> {
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }

        let result = self.current;
        self.current.x = self.current.x + T::one();
        if self.current.x == self.end.x {
            self.current.x = self.start.x;
            self.current.y = self.current.y + T::one();
            if self.current.y == self.end.y {
                self.finished = true;
            }
        }

        Some(result)
    }
}