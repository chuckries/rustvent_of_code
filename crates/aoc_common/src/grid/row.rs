use std::ops::Index;

use crate::Grid;

pub struct RowsIter<'a, T> {
    front: usize,
    back: usize,
    grid: &'a Grid<T>
}

impl<'a, T> RowsIter<'a, T> {
    pub (in crate::grid) fn new(grid: &'a Grid<T>) -> Self {
        Self {
            front: 0,
            back: grid.height(),
            grid,
        }
    }
}

impl<'a, T> Iterator for RowsIter<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let idx = self.front;
        self.front = idx + 1;
        Some(Row::new(idx, self.grid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.back - self.front;
        (size, Some(size))
    }
}

impl<'a, T> DoubleEndedIterator for RowsIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back <= self.front {
            return None;
        }

        self.back -= 1;
        Some(Row::new(self.back, self.grid))
    }
}

impl<'a, T> ExactSizeIterator for RowsIter<'a, T> { }

pub struct Row<'a, T> {
    col: usize,
    grid: &'a Grid<T>
}

impl<'a, T> Row<'a, T> {
    pub (in crate::grid) fn new(col: usize, grid: &'a Grid<T>) -> Self {
        Self {
            col,
            grid
        }
    }

    pub fn len(&self) -> usize {
        self.grid.width()
    }

    pub fn iter(&self) -> RowIter<'a, T> {
        RowIter::new(self.col, self.grid)
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid.grid[self.col][index]
    }
}

pub struct RowIter<'a, T> {
    front: usize,
    back: usize,
    col: usize,
    grid: &'a Grid<T>,
}

impl<'a, T> RowIter<'a, T> {
    fn new(col: usize, grid: &'a Grid<T>) -> Self {
        Self {
            front: 0,
            back: grid.width(),
            col,
            grid
        }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let idx = self.front;
        self.front = idx + 1;

        Some(&self.grid.grid[self.col][idx])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.back - self.front;
        (size, Some(size))
    }
}

impl<'a, T> DoubleEndedIterator for RowIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back <= self.front {
            return None;
        }

        self.back -= 1;
        Some(&self.grid.grid[self.col][self.back])
    }
}

impl<'a, T> ExactSizeIterator for RowIter<'a, T> { }

#[cfg(test)]
mod test {
    use crate::{Grid, IteratorExt};

    #[test]
    fn row_iter_rev() {
        let grid = Grid::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        let mut rows = grid.rows();
        assert_eq!(rows.next().unwrap().iter().rev().copied().to_vec(), vec![3, 2, 1]);
        assert_eq!(rows.next().unwrap().iter().rev().copied().to_vec(), vec![6, 5, 4]);
        assert_eq!(rows.next().unwrap().iter().rev().copied().to_vec(), vec![9, 8, 7]);
        assert!(rows.next().is_none());
        
        let grid = Grid::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        let mut row = grid.rows().next().unwrap().iter();
        assert_eq!(Some(&1), row.next());
        assert_eq!(Some(&3), row.next_back());
        assert_eq!(Some(&2), row.next_back());
        assert_eq!(None, row.next());
    }
}