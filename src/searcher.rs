struct Searcher {
    tokens: Vec<InvertedIndex>,
    unions: Vec<&dyn Corpus>,
}

impl Searcher {
    fn find_unions(&self) -> &self {}
    fn sort_unions_by_relevancy(&self) -> &self {}
}