use std::{ops::{Index, IndexMut}};

use num_traits::{PrimInt};

use crate::{Vec2, Vec2us, grid2::{grid2_enumerate::GridEnumerate, row::Row}};

pub use builder::Grid2Builder;

mod builder;
mod grid2_enumerate;
mod row;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid2<T> {
    pub(self) width: usize,
    height: usize,
    buffer: Vec<T>,
}

impl<T> Grid2<T> {
    #[inline]
    fn pos_to_idx(&self, p: Vec2us) -> usize {
        p.y * self.width() + p.x
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn dims(&self) -> Vec2us {
        Vec2us::new(self.width, self.height)
    }

    pub fn contains_pos(&self, p: Vec2us) -> bool {
        p.x < self.width() && p.y < self.height()
    }

    pub fn from_buffer(width: usize, height: usize, buffer: Vec<T>) -> Self {
        if width * height != buffer.len() {
            panic!("bad dimensions");
        } 

        Self {
            width,
            height,
            buffer
        }
    }

    pub fn row(&self, row: usize) -> Row<'_, T> {
        Row::new(self, row)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.buffer.iter_mut()
    }

    pub fn enumerate(&self) -> GridEnumerate<'_, T> {
        GridEnumerate::new(self.buffer.as_slice(), self.dims())
    }
}

impl<T, U: PrimInt> Index<Vec2<U>> for Grid2<T> {
    type Output = T;

    fn index(&self, index: Vec2<U>) -> &Self::Output {
        let index = index.cast();
        if self.contains_pos(index) {
            unsafe { self.buffer.get_unchecked(self.pos_to_idx(index)) }
        } else {
            panic!("out of bounds")
        }
    }
}

impl<T, U: PrimInt> IndexMut<Vec2<U>> for Grid2<T> {
    fn index_mut(&mut self, index: Vec2<U>) -> &mut Self::Output {
        let index = index.cast();
        if self.contains_pos(index) {
            let index = self.pos_to_idx(index);
            unsafe { self.buffer.get_unchecked_mut(index) }
        } else {
            panic!("out of bounds")
        }
    }
}