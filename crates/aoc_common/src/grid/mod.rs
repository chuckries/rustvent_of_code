use std::{fmt::Display, ops::{Index, IndexMut}};
use num_traits::NumCast;

use crate::{Vec2, Vec2us, file_lines, grid::row::{Row, RowsIter}};

mod row;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }

    pub fn file_as_grid<F>(path: &str, f: &mut F) -> Grid<T> 
    where
        F: FnMut(u8, Vec2us) -> T
    {
        let mut rows: Vec<Vec<T>> = Vec::new();
        let mut j = 0;
        for line in file_lines(path) {
            let mut cells: Vec<T> = Vec::new();
            let mut i = 0;
            for b in line.bytes() {
                cells.push(f(b, (i, j).into()));
                i += 1;
            }
            j += 1;
            rows.push(cells);
        }
        Grid::new(rows)
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    pub fn bounds(&self) -> Vec2us {
        Vec2us::new(self.width(), self.height())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.enumerate().map(|(_, t)| t)
    }

    pub fn row(&self, idx: usize) -> Row<'_, T> {
        Row::new(idx, self)
    }

    pub fn rows(&self) -> RowsIter<'_, T> {
        RowsIter::new(self)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Vec2us, &T)> {
        (0..self.grid.len()).flat_map(|j| {
            (0..self.grid[0].len()).map(move |i| (i, j))
        }).map(|p| {
            (p.into(), &self[p])
        })
    }

    pub fn same_of_type<U>(&self) -> Grid<U> 
    where
        U: Default + Clone
    {
        Grid::with_dimensions(self.bounds())
    }

    pub fn adjacent(&self, p: Vec2us) -> impl Iterator<Item = &T> {
        if !p.is_in_bounds(self.bounds()) {
            panic!("out of bounds");
        }

        p.adjacent_bounded(&self.bounds()).map(|adj| &self[adj])
    }

    pub fn adjacent_enumerate(&self, p: Vec2us) -> impl Iterator<Item = (Vec2us, &T)> {
        if !p.is_in_bounds(self.bounds()) {
            panic!("out of bounds");
        }

        p.adjacent_bounded(&self.bounds()).map(|adj| (adj, &self[adj]))
    }

    pub fn surrounding(&self, p: Vec2us) -> impl Iterator<Item = &T> {
        if !p.is_in_bounds(self.bounds()) {
            panic!("out of bounds");
        }

        p.surrouding_bounded(&self.bounds()).map(|adj| &self[adj])
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn with_dimensions(dims: Vec2us) -> Self {
        Self {
            grid: vec![vec![T::default(); dims.x]; dims.y]
        }
    }
}

impl<T, U> Index<Vec2<U>> for Grid<T> 
where
    U: NumCast
{
    type Output = T;

    fn index(&self, index: Vec2<U>) -> &Self::Output {
        &self.grid[<usize as NumCast>::from(index.y).unwrap()][<usize as NumCast>::from(index.x).unwrap()]
    }
}

impl<T, U> IndexMut<Vec2<U>> for Grid<T>
where 
    U: NumCast
{
    fn index_mut(&mut self, index: Vec2<U>) -> &mut Self::Output {
        &mut self.grid[<usize as NumCast>::from(index.y).unwrap()][<usize as NumCast>::from(index.x).unwrap()]
    }
}

impl<T, U1,U2,> Index<(U1, U2)> for Grid<T>
where 
    U1: NumCast,
    U2: NumCast,
{
    type Output = T;

    fn index(&self, index: (U1, U2)) -> &Self::Output {
        &self.grid[<usize as NumCast>::from(index.1).unwrap()][<usize as NumCast>::from(index.0).unwrap()]
    }
}

impl<T, U1, U2> IndexMut<(U1, U2)> for Grid<T>
where 
    U1: NumCast,
    U2: NumCast,
{
    fn index_mut(&mut self, index: (U1, U2)) -> &mut Self::Output {
        &mut self.grid[<usize as NumCast>::from(index.1).unwrap()][<usize as NumCast>::from(index.0).unwrap()]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self {
            grid: value
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rows in self.grid.iter() {
            for c in rows.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}