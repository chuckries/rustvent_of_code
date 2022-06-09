use crate::{vec2, Vec2us};

pub struct Map2<T> {
    bounds: Vec2us,
    map: Vec<Vec<T>>,
}

impl<T: Clone + Default> Map2<T> {
    fn new(bounds: Vec2us) -> Self {
        Self { 
            bounds,
            map: vec![vec![T::default(); bounds.x]; bounds.y]
        }
    }
}

impl<T> std::ops::Index<Vec2us> for Map2<T> {
    type Output = T;

    fn index(&self, index: Vec2us) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

impl<T> std::ops::IndexMut<Vec2us> for Map2<T> {
    fn index_mut(&mut self, index: Vec2us) -> &mut Self::Output {
        &mut self.map[index.y][index.x]
    }
}

pub struct Iter<'a, T> 
    where T: 'a
{
    current: vec2::Iter<usize>,
    map: &'a Map2<T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.current.next() {
            Some(&self.map[p])
        } else {
            None
        }
    }
}