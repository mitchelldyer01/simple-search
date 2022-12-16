struct TermPerDocument {
    id: u32,
    term: String,
    term_frequency: u32,
}

struct InvertedIndex {
    token: String,
    documents: Vec<TermPerDocument>,
    inv_doc_freq: u32,
    relevancy: u32,
}

impl InvertedIndex {
    fn calc_inv_doc_freq(&self) -> &self {}
    fn calc_relevancy(&self) -> &self {}
    fn store_on_disk(&self) -> &self {}
    fn find_on_disk(&self) -> &self {}
}