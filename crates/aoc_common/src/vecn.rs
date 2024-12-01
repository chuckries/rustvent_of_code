use std::ops::*;

#[derive(Copy, Clone)]
pub struct VecN<const N: usize, T> {
    vec: [T; N]
}

impl<const N: usize, T> VecN<N, T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.vec.into_iter()
    }
}

impl<const N: usize, T: Copy + Add<Output = T>> Add for VecN<N, T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..self.vec.len() {
            self.vec[i]  = self.vec[i] + rhs.vec[i];
        }
        self
    }
}


impl<const N: usize, T: Copy + Mul<Output = T>> Mul<T> for VecN<N, T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for i in self.vec.iter_mut() {
            *i = *i * rhs;
        }
        self
    }
}

impl<const N: usize, T: Copy> Index<usize> for VecN<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<const N: usize, T: Copy> IndexMut<usize> for VecN<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<const N: usize, T: Default + Copy> Default for VecN<N, T> {
    fn default() -> Self {
        Self { vec: [T::default(); N] }
    }
}