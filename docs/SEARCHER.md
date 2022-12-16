# Searcher

The `searcher` will filter the query, lookup all
matching indices, find union of documents in
all matching indices, order them by relevancy score,
and fetch actual documents. 

## Structure

```rust
struct QueryFilter {
    query: String,
}

impl Filter for QueryFilter {} // see INDEXER.md

struct Searcher {
    tokens: Vec<InvertedIndex>,
    unions: Vec<&dyn Corpus>,
}

impl Searcher {
    fn find_unions(&self) -> &self {}
    fn sort_unions_by_relevancy(&self) -> &self {}
}
```
