use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div};
use num_traits::{PrimInt, Signed, Num, NumCast, ToPrimitive};

pub type Vec2us = Vec2<usize>;
pub type Vec2u8 = Vec2<u8>;
pub type Vec2u16 = Vec2<u16>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;
pub type Vec2u128 = Vec2<u128>;
pub type Vec2is = Vec2<isize>;
pub type Vec2i8 = Vec2<i8>;
pub type Vec2i16 = Vec2<i16>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2i128 = Vec2<i128>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec2f64 = Vec2<f64>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Vec2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vec2<T> {
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
}

impl<T: PrimInt> Vec2<T> {
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

impl<T: Num + Signed> Vec2<T> {
    pub fn signum(&self) -> Self {
        Self { x: self.x.signum(), y: self.y.signum() }
    }
}

impl<T: Num + ToPrimitive + Copy> Vec2<T> {
    pub fn cast<U: Num + NumCast>(&self) -> Vec2<U> {
        Vec2 { x: U::from(self.x).unwrap(), y: U::from(self.y).unwrap() }
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
manhattan_unsigned!(u8);
manhattan_unsigned!(u16);
manhattan_unsigned!(u32);
manhattan_unsigned!(u64);
manhattan_unsigned!(u128);
manhattan_signed!(isize);
manhattan_signed!(i8);
manhattan_signed!(i16);
manhattan_signed!(i32);
manhattan_signed!(i64);
manhattan_signed!(i128);

impl <T: Num + std::fmt::Display> std::fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl <T: Num + Copy + Neg<Output = T>> Vec2<T> {
    pub fn rotate_left(&self) -> Self {
        Self { x: self.y, y: -self.x }
    }

    pub fn rotate_right(&self) -> Self {
        Self { x: -self.y, y: self. x }
    }
}

impl<T: Num + Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl <T: Num> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: Num + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl <T: Num> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl <T: Num + Copy> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl <T: Num + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl <T: Num + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl <T: Num + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T: Num> From<(T, T)> for Vec2<T> {
    fn from(v: (T, T)) -> Self {
        Vec2 { x: v.0, y: v.1 }
    }
}

impl<T: Num> From<Vec2<T>> for (T, T) {
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