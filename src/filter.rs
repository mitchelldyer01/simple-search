trait Filter {
    fn tokenize(&self) -> &self {}
    fn to_lowercase(&self) -> &self {}
    fn remove_punctuation(&self) -> &self {}
    fn remove_stopwords(&self) -> &self {}
    fn apply_stemming(&self) -> &self {}
    fn parse_text(&self) -> &self {}
    fn count_frequencies(&self) -> Vec<TermPerDocument> {}
}

struct CorpusFilter {
    document: &dyn Corpus,
}

impl Filter for CorpusFilter {}

struct QueryFilter {
    query: String,
}

impl Filter for QueryFilter {}
