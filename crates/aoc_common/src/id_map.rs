use std::collections::HashMap;

pub struct IdMap {
    dict: HashMap<String, usize>,
    idx_to_key: Vec<String>,
}

impl IdMap {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
            idx_to_key: Vec::new(),
        }
    }

    pub fn get_or_insert(&mut self, id: &str) -> usize {
        let next_idx = self.dict.len();
        *self.dict.entry(id.to_string()).or_insert_with(|| {
            self.idx_to_key.push(id.to_string());
            next_idx
        })
    }

    pub fn get(&self, id: &str) -> Option<usize> {
        self.dict.get(id).and_then(|id| Some(*id))
    }

    pub fn get_key(&self, idx: usize) -> &str {
        &self.idx_to_key[idx]
    }

    pub fn len(&self) -> usize {
        self.dict.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &usize)> {
        self.dict.iter()
    }
}