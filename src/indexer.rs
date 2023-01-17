use std::collections::HashMap;

pub struct InvertedIndex {
    index: HashMap<String, Vec<String>>
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex{
            index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, token: String, id: String) {
        let entry = self.index.entry(token).or_insert(Vec::new());
        entry.push(id);
    }
}