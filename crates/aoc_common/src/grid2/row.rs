use std::ops::Index;

use crate::Grid2;

pub struct Row<'a, T> {
    slice: &'a [T]
}

impl<'a, T> Row<'a, T> {
    pub(super) fn new(grid: &'a Grid2<T>, row: usize) -> Self {
        if row >= grid.width() {
            panic!("out of bounds")
        }
            
        let start = row * grid.width;
        Self {
            slice: &grid.buffer[start .. start + grid.width]
        }
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.slice.iter()
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slice[index]
    }
}