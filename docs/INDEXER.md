# Indexer

The `indexer` will ingest the document & its metadata,
tokenize the text, filter the text, then create an inverted
index for each token with frequency score.

## Structure

```rust
struct Pdf {
    id: u32,
    title: String,
    body: String,
    date: String,
    author: String,
}

trait Corpus {
    fn ingest(&self) -> &self {}
    fn ingest_document(&self) -> &self {}
    fn ingest_title(&self) -> &self {}
    fn ingest_body(&self) -> &self {}
    fn ingest_date(&self) -> &self {}
    fn ingest_author(&self) -> &self {}
}

impl Corpus for Pdf {}

struct CorpusFilter {
    document: &dyn Corpus,
}

trait Filter {
    fn tokenize(&self) -> &self {}
    fn to_lowercase(&self) -> &self {}
    fn remove_punctuation(&self) -> &self {}
    fn remove_stopwords(&self) -> &self {}
    fn apply_stemming(&self) -> &self {}
    fn parse_text(&self) -> &self {}
    fn count_frequencies(&self) -> Vec<TermPerDocument> {}
}

impl Filter for CorpusFilter {}

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
```
