use std::{borrow::Borrow, collections::HashMap, hash::Hash};

pub struct IdMap<T = String> {
    dict: HashMap<T, usize>,
    idx_to_key: Vec<T>,
}

impl<T> IdMap<T> {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
            idx_to_key: Vec::new(),
        }
    }

    pub fn get_key(&self, idx: usize) -> &T {
        &self.idx_to_key[idx]
    }

    pub fn len(&self) -> usize {
        self.dict.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.dict.iter()
    }
}

impl<T: Eq + Hash> IdMap<T>
{
    pub fn get_or_insert<U>(&mut self, id: &U) -> usize 
    where 
        T: Borrow<U>,
        U: Eq + Hash + ToOwned<Owned = T> + ?Sized,
    {
        if let Some(found) = self.dict.get(id) {
            *found
        } else {
            let idx = self.dict.len();
            self.idx_to_key.push(id.to_owned());
            self.dict.insert(id.to_owned(), idx);
            idx
        }
    }

    pub fn get<U>(&self, id: &U) -> Option<usize> 
    where
        T: Borrow<U>,
        U: Eq + Hash + ?Sized,
    {
        self.dict.get(id).cloned()
    }
}