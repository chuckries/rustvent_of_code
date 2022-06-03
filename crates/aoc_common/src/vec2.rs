use num_traits::PrimInt;

pub type Vec2us = Vec2<usize>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2u64 = Vec2<u64>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec2<T: PrimInt> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn adjacent(&self) -> impl Iterator<Item = Self> {
        [
            Self { x: self.x - T::one(), y: self.y            },
            Self { x: self.x + T::one(), y: self.y            },
            Self { x: self.x           , y: self.y - T::one() },
            Self { x: self.x           , y: self.y + T::one() },
        ].into_iter()
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

impl<T: PrimInt> From<(T, T)> for Vec2<T> {
    fn from(p: (T, T)) -> Self {
        Vec2 { x: p.0, y: p.1 }
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