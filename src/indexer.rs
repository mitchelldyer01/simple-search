use std::collections::HashMap;

pub struct InvertedIndex {
    index: HashMap<String, HashMap<String, i32>>
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex{
            index: HashMap::new(),
        }
    }

    pub fn update_document(&mut self, token: String, id: String) {
        let documents = self.index.entry(token).or_insert(HashMap::new());
        *documents.entry(id).or_insert(0) += 1;
    }

    pub fn print(&mut self) {
        self.index.iter().for_each(|(k, v)| println!("{}: {:?}", k, v));
    }
}