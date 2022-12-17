use std::{collections::{HashSet, VecDeque}, cmp::Ordering, hash::Hash};

pub trait IteratorExt: Iterator
{
    fn to_vec(self) -> Vec<Self::Item>
        where Self: Sized
    {
        Vec::from_iter(self)
    }

    fn to_set(self) -> HashSet<Self::Item>
    where 
        Self: Sized, 
        Self::Item: Eq + Hash
    {
        HashSet::from_iter(self)
    }

    fn to_vec_deque(self) -> VecDeque<Self::Item>
        where Self: Sized
    {
        VecDeque::from_iter(self)
    }

    fn sorted(self) -> std::vec::IntoIter<Self::Item>
    where 
        Self: Sized, 
        Self::Item: Ord
    {
        let mut v = Vec::from_iter(self);
        v.sort();
        v.into_iter()
    }

    fn sorted_by<F>(self, f: F) -> std::vec::IntoIter<Self::Item>
    where 
        Self: Sized, 
        F: FnMut(&Self::Item, &Self::Item) -> Ordering
    {
        let mut v = Vec::from_iter(self);
        v.sort_by(f);
        v.into_iter()
    }

    fn sorted_by_key<K, F>(self, f: F) -> std::vec::IntoIter<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K
    {
        let mut v = Vec::from_iter(self);
        v.sort_by_key(f);
        v.into_iter()
    }

    fn sorted_by_cached_key<K, F>(self, f: F) -> std::vec::IntoIter<Self::Item>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K
    {
        let mut v = Vec::from_iter(self);
        v.sort_by_cached_key(f);
        v.into_iter()
    }

    fn unique(self) -> std::collections::hash_set::IntoIter<Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + Hash
    {
        self.to_set().into_iter()
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator { }