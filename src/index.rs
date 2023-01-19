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

    pub fn search(&mut self, terms: Vec<String>) {
        let results: HashMap<String, i32> = terms
            .iter()
            .filter_map(|token| self.index.get(token))
            .fold(HashMap::new(), |mut acc, b|{
                for (result, count) in b {
                    *acc.entry(result.clone()).or_insert(0) += count;
                }
                acc
            });

        let mut sorted_results: Vec<(&String, &i32)> = results.iter().collect();
        sorted_results.sort_by(|a, b| b.1.cmp(a.1));

        for (result, count) in sorted_results {
            println!("{}: {}", result, count);
        }
    }
}