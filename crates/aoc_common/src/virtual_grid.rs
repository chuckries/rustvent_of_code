use std::ops::{Index, IndexMut};

use crate::Vec2us;

pub struct VirtualGrid<T> {
    grid: Vec<Vec<Option<T>>>,
}

impl<T> VirtualGrid<T> {
    pub fn new() -> Self {
        Self {
            grid: Vec::new()
        }
    }

    pub fn with_capacity(size: Vec2us) -> Self {
        let mut grid = Self::new();
        grid.ensure_size(size);
        grid
    }

    pub fn insert(&mut self, idx: Vec2us, item: T) -> Option<T> {
        self.ensure_size(idx);
        self[idx].replace(item)
    }

    pub fn entry(&mut self, idx: Vec2us) -> Entry<T> {
        self.ensure_size(idx);
        Entry { entry: &mut self[idx] }
    }

    fn ensure_size(&mut self, size: Vec2us) {
        if self.grid.is_empty() {
            for _ in 0..=size.y {
                let mut row: Vec<Option<T>> = Vec::new();
                row.resize_with(size.x + 1, || None);
                self.grid.push(row);
            }
        } else {
            let mut x = self.grid[0].len();
            if size.x >= x {
                x = usize::max(size.x + 1, self.grid[0].len() * 2);

                for row in self.grid.iter_mut() {
                    row.resize_with(x, || None);
                }
            }

            if size.y >= self.grid.len() {
                let y = usize::max(size.y + 1, self.grid.len() * 2);

                while self.grid.len() <= y {
                    let mut row: Vec<Option<T>> = Vec::new();
                    row.resize_with(x, || None);
                    self.grid.push(row);
                }
            }
        }
    }
}

impl<T> Index<Vec2us> for VirtualGrid<T> {
    type Output = Option<T>;

    fn index(&self, index: Vec2us) -> &Self::Output {
        if index.y >= self.grid.len() || index.x >= self.grid[0].len() {
            &None
        } else {
            &self.grid[index.y][index.x]
        }
    }
}

impl<T> IndexMut<Vec2us> for VirtualGrid<T> {
    fn index_mut(&mut self, index: Vec2us) -> &mut Self::Output {
        self.ensure_size(index);
        &mut self.grid[index.y][index.x]
    }
}

pub struct Entry<'a, T>
{
    entry: &'a mut Option<T>
}

impl<'a, T> Entry<'a, T> {
    pub fn or_insert(self, i: T) -> &'a mut T {
        self.entry.get_or_insert(i)
    }

    pub fn or_insert_with<F>(self, f: F) -> &'a mut T 
        where F: FnOnce() -> T
    {
        self.entry.get_or_insert_with(f)
    }
}

impl<'a, T: Default> Entry<'a, T> {
    pub fn or_default(self) -> &'a mut T {
        self.entry.get_or_insert(T::default())
    }
}