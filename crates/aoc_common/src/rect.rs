use num_traits::{PrimInt};

use crate::Vec2;

pub type RectUs = Rect<usize>;
pub type RectI64 = Rect<i64>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Rect<T: PrimInt> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: PrimInt> Rect<T> {
    pub fn from_points(p0: Vec2<T>, p1: Vec2<T>) -> Self {
        let x = p0.x.min(p1.x);
        let y = p0.y.min(p1.y);
        let width = p0.x.max(p1.x) - x;
        let height = p0.y.max(p1.y) - y;

        Self { x, y, width, height }
    }

    pub fn from_point_size(p0: Vec2<T>, size: Vec2<T>) -> Self {
        Self {
            x: p0.x,
            y: p0.y,
            width: size.x,
            height: size.y,
        }
    }

    pub fn from_size(size: Vec2<T>) -> Self {
        Self::from_point_size(Vec2::zero(), size)
    }

    pub fn bounding<I>(it: I) -> Self
    where
        I: Iterator<Item = Vec2<T>>
    {
        let mut min = Vec2 { x: T::max_value(), y: T::max_value() };
        let mut max = Vec2 { x: T::min_value(), y: T::min_value() };

        for i in it {
            if i.x < min.x {
                min.x = i.x;
            }
            if i.y < min.y {
                min.y = i.y;
            }
            if i.x > max.x {
                max.x = i.x;
            }
            if i.y > max.y {
                max.y = i.y;
            }
        }

        Self::from_points(min, max)
    }

    pub fn is_unit(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn width_mut(&mut self) -> &mut T {
        &mut self.width
    }

    pub fn height(&self) -> T {
        self.height
    }

    pub fn height_mut(&mut self) -> &mut T {
        &mut self.height
    }

    pub fn left(&self) -> T {
        self.x
    }

    pub fn top(&self) -> T { 
        self.y
    }

    pub fn right(&self) -> T {
        self.x + self.width
    }

    pub fn bottom(&self) -> T {
        self.y + self.height
    }

    pub fn top_left(&self) -> Vec2<T> {
        Vec2::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> Vec2<T> {
        Vec2::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> Vec2<T> {
        Vec2::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Vec2<T> {
        Vec2::new(self.right(), self.bottom())
    }

    pub fn corners(&self) -> [Vec2<T>; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.bottom_left(),
            self.bottom_right()
        ]
    }

    pub fn subdivide(&self) -> Option<std::vec::IntoIter<Self>> {
        if self.is_unit() {
            None
        } else {
            let mut divisions: Vec<Self> = Vec::with_capacity(4);

            let split_x = self.width > T::zero();
            let split_y = self.height > T::zero();

            let mid_x = self.x + self.width / T::from(2).unwrap();
            let mid_y = self.y + self.height / T::from(2).unwrap();

            divisions.push(Self::from_points((self.x, self.y).into(), (mid_x, mid_y).into()));
            if split_x { divisions.push(Self::from_points((mid_x + T::one(), self.y).into(), (self.right(), mid_y).into())) }
            if split_y { divisions.push(Self::from_points((self.x, mid_y + T::one()).into(), (mid_x, self.bottom()).into())) }
            if split_x && split_y { divisions.push(Self::from_points((mid_x + T::one(), mid_y + T::one()).into(), (self.right(), self.bottom()).into())) }

            Some(divisions.into_iter())
        }
    }

    pub fn area(&self) -> T {
        self.width * self.height
    }
}