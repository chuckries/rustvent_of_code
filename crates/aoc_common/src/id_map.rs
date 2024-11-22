use std::collections::HashMap;

pub struct IdMap {
    dict: HashMap<String, usize>
}

impl IdMap {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new()
        }
    }

    pub fn get_or_insert(&mut self, id: &str) -> usize {
        let next_id = self.dict.len();
        *self.dict.entry(id.to_string()).or_insert(next_id)
    }

    pub fn get(&self, id: &str) -> Option<usize> {
        self.dict.get(id).and_then(|id| Some(*id))
    }

    pub fn len(&self) -> usize {
        self.dict.len()
    }
}