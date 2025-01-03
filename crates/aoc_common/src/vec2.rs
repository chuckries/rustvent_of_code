use std::{cmp::Ordering, fmt::Debug, iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};
use num_traits::{PrimInt, Signed};

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }

    pub fn one() -> Self {
        Self { x: T::one(), y: T::one() }
    }

    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero() }
    }
    
    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one() }
    }

    pub fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
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

    pub fn surrounding_unbounded(&self) -> impl Iterator<Item = Self> {
        [
            Self { x: self.x - T::one(), y: self.y - T::one() },
            Self { x: self.x - T::one(), y: self.y            },
            Self { x: self.x - T::one(), y: self.y + T::one() },
            Self { x: self.x           , y: self.y - T::one() },
            Self { x: self.x           , y: self.y + T::one() },
            Self { x: self.x + T::one(), y: self.y - T::one() },
            Self { x: self.x + T::one(), y: self.y            },
            Self { x: self.x + T::one(), y: self.y + T::one() },
        ].into_iter()
    }

    pub fn surrouding_bounded(&self, bounds: &Self) -> impl Iterator<Item = Self> {
        let mut sur = Vec::with_capacity(8);
    
        if self.x > T::zero() && self.y > T::zero()                         { sur.push(Self { x: self.x - T::one(), y: self.y - T::one() }); }
        if self.x > T::zero()                                               { sur.push(Self { x: self.x - T::one(), y: self.y            }); }
        if self.x > T::zero() && self.y < bounds.y - T::one()               { sur.push(Self { x: self.x - T::one(), y: self.y + T::one() }); }
        if self.y > T::zero()                                               { sur.push(Self { x: self.x           , y: self.y - T::one() }); }
        if self.y < bounds.y - T::one()                                     { sur.push(Self { x: self.x           , y: self.y + T::one() }); }
        if self.x < bounds.x - T::one() && self.y > T::zero()               { sur.push(Self { x: self.x + T::one(), y: self.y - T::one() }); }
        if self.x < bounds.x - T::one()                                     { sur.push(Self { x: self.x + T::one(), y: self.y            }); }
        if self.x < bounds.x - T::one() && self.y < bounds.y - T::one()     { sur.push(Self { x: self.x + T::one(), y: self.y + T::one() }); }
        sur.into_iter()
    }

    pub fn bounds_from_zero_inclusive<I: Iterator<Item = Self>>(it: I) -> Self {
        let mut bounds = Self::zero();

        for i in it {
            if i.x > bounds.x {
                bounds.x = i.x;
            }
            if i.y > bounds.y {
                bounds.y = i.y;
            }
        }

        bounds
    }

    pub fn bounds_from_zero_exclusive<I: Iterator<Item = Self>>(it: I) -> Self {
        Self::bounds_from_zero_inclusive(it) + Self { x: T::one(), y: T::one() }
    }

    pub fn max_value() -> Self {
        Self { x: T::max_value(), y: T::max_value() }
    }

    pub fn min_value() -> Self {
        Self { x: T::min_value(), y: T::min_value() }
    }

    pub fn cast<U: PrimInt>(&self) -> Vec2<U> {
        Vec2 { x: U::from(self.x).unwrap(), y: U::from(self.y).unwrap() }
    }

    /// returns an iterator that yields every point in the square defined by 
    /// self and p1 (inclusive). Order will be top left to bottom right, regardless
    /// of the the two points provided
    pub fn area(&self, p1: Self) -> Iter<T> {
        let mut min = *self;
        let mut max = p1;

        if min.x > max.x {
            (min.x, max.x) = (max.x, min.x);
        }

        if min.y > max.y {
            (min.y, max.y) = (max.y, min.y);
        }

        Iter::new(min, max + Self::one())
    }

    /// return an iterator that yields every point in the square between (0, 0) and the 
    /// self (exclusive). Only yields points in the first quadrant.
    pub fn iter(&self) -> Iter<T> {
        Iter::new(Self::zero(), *self)
    }

    // sorts top to bottom, left to right for standard grid coords
    pub fn grid_ordering(&self, rhs: &Self) -> Ordering {
        let mut ord = self.y.cmp(&rhs.y);
        if ord == Ordering::Equal {
            ord = self.x.cmp(&rhs.x);
        }
        ord
    }
}

impl<T:PrimInt + Signed> Vec2<T> {
    pub fn signum(&self) -> Self {
        Self { x: self.x.signum(), y: self.y.signum() }
    }

    pub fn north_of(&self) -> Self {
        *self - Self::unit_y()
    }

    pub fn south_of(&self) -> Self {
        *self + Self::unit_y()
    }

    pub fn west_of(&self) -> Self {
        *self - Self::unit_x()
    }

    pub fn east_of(&self) -> Self {
        *self + Self::unit_x()
    }

    pub fn north_east_of(&self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y - T::one(),
        }
    }

    pub fn north_west_of(&self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y - T::one(),
        }
    }

    pub fn south_east_of(&self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y + T::one(),
        }
    }

    pub fn south_west_of(&self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y + T::one(),
        }
    }

    pub fn north_by(&self, dist: T) -> Self {
        Self {
            x: self.x,
            y: self.y - dist,
        }
    }

    pub fn south_by(&self, dist: T) -> Self {
        Self {
            x: self.x,
            y: self.y + dist,
        }
    }

    pub fn west_by(&self, dist: T) -> Self {
        Self {
            x: self.x - dist,
            y: self.y,
        }
    }

    pub fn east_by(&self, dist: T) -> Self {
        Self {
            x: self.x + dist,
            y: self.y,
        }
    }

    pub fn unit_dirs() -> impl Iterator<Item = Self> {
        Self::zero().adjacent().into_iter()
    }

    pub fn unit_dirs_and_diags() -> impl Iterator<Item = Self> {
        Self::zero().surrounding_unbounded().into_iter()
    }
}

impl<T: PrimInt + FromStr> Vec2<T>
where 
    <T as FromStr>::Err: Debug
{
    pub fn parse<S: AsRef<str>>(x: S, y: S) -> Self {
        Self {
            x: x.as_ref().parse::<T>().unwrap(),
            y: y.as_ref().parse::<T>().unwrap()
        }
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

impl <T: PrimInt + std::fmt::Display> std::fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl <T: PrimInt + Neg<Output = T>> Vec2<T> {
    pub fn rotated_left(&self) -> Self {
        Self { x: self.y, y: -self.x }
    }

    pub fn rotated_right(&self) -> Self {
        Self { x: -self.y, y: self. x }
    }

    pub fn rotate_left(&mut self) {
        (self.x, self.y) = (self.y, -self.x);
    }

    pub fn rotate_right(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }
}

impl<T: PrimInt + Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl <T: PrimInt> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: PrimInt> Add for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: PrimInt> Add<&Self> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: PrimInt> Add<&Self> for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: PrimInt> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl <T: PrimInt> AddAssign for &mut Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl <T: PrimInt> AddAssign<&Self> for Vec2<T> {
    fn add_assign(&mut self, rhs: &Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl <T: PrimInt> AddAssign<&Self> for &mut Vec2<T> {
    fn add_assign(&mut self, rhs: &Self) {
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

impl <T: PrimInt> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<T: PrimInt> Sum for Vec2<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
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

impl<T: PrimInt> FromIterator<T> for Vec2<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        }
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