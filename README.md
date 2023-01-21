# simple-search

A simple search engine for plaintext documents.
Filters plaintext documents into an inverted index.
Search ranks results by the number of occurences
of search terms per document.

The list of plaintext documents it indexes
is in [list][].

## Run

```bash
cargo run
```

## To-do

Some things that could greatly improve performance

- Write the inverted index to a memory-mapped file on disk
- Convert `Vec<>` to `HashSet` for terms
- Use a more space-efficient implementation of the
  inverted index (sparse matrix)

[list]: ./list

