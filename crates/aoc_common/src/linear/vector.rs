use std::{ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}, process::Output};

pub struct Vector<T>
{
    buff: Vec<T>
}

impl<T> Vector<T> {
    fn len(&self) -> usize {
        self.buff.len()
    }
}

impl<T: Add<Output = T> + Copy> Add<&Self> for Vector<T> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!();
        }

        for (r, l) in self.buff.iter_mut().zip(rhs.buff.iter()) {
            *r = *r + *l;
        }

        self
    }
}

impl<T: AddAssign + Copy> AddAssign<&Self> for Vector<T> {
    fn add_assign(&mut self, rhs: &Self) {
        if self.len() != rhs.len() {
            panic!();
        }
        
        for (r, l) in self.buff.iter_mut().zip(rhs.buff.iter()) {
            *r += *l;
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<&Self> for Vector<T> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!();
        }

        for (r, l) in self.buff.iter_mut().zip(rhs.buff.iter()) {
            *r = *r - *l
        }

        self
    }
}

impl<T: SubAssign + Copy> SubAssign<&Self> for Vector<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        if self.len() != rhs.len() {
            panic!();
        }
        
        for (r, l) in self.buff.iter_mut().zip(rhs.buff.iter()) {
            *r -= *l;
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for n in self.buff.iter_mut() {
            *n = *n * rhs;
        }

        self
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        for n in self.buff.iter_mut() {
            *n *= rhs
        }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Vector<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for n in self.buff.iter_mut() {
            *n = -*n;
        }
        self
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            buff: value
        }
    }
}

impl<T: Clone> From<&[T]> for Vector<T> {
    fn from(value: &[T]) -> Self {
        Self {
            buff: value.to_vec(),
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    fn from(value: [T; N]) -> Self {
        Self {
            buff: value.into(),
        }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buff[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buff[index]
    }
}