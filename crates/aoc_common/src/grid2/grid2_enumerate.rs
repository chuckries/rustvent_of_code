use std::iter::FusedIterator;

use crate::Vec2us;

pub struct GridEnumerate<'a, T> {
    val_iter: std::slice::Iter<'a, T>,
    pos_iter: crate::vec2::Iter<usize>,
}

impl<'a, T> GridEnumerate<'a, T> {
    pub(super) fn new(slice: &'a [T], dims: Vec2us) -> Self {
        Self {
            val_iter: slice.iter(),
            pos_iter: dims.iter(),
        }
    }
}

impl<'a, T> Iterator for GridEnumerate<'a, T> {
    type Item = (Vec2us, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.val_iter.next().and_then(|v| Some((self.pos_iter.next().unwrap(), v)))
    }
}

impl<'a, T> FusedIterator for GridEnumerate<'a, T> { }