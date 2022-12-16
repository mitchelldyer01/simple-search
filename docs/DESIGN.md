# Design

We will process the text
- Ingest the text & its metadata
    - Body
    - Title
    - Author
    - Date
- Tokenize the body
- Filter the body
    - Lowercase it
    - Remove punctuation
    - Remove common words
    - Apply stemming

We will index the parsed text
- Inverted index of the body
    - The key is the keyword
    - The value is a composition of
        - a list of the documents
        - with the frequency the keyword appears within that document
            - multiply the term frequency by the inverse document frequency
                - logarithm(# of documents / # of documents containing term)
        

We will search the indices of parsed text
- Parse the search text the same way we parsed the body
- Lookup all the matching indices
- Find the documents present in all of the selected indices
- Order them by the sum of their frequency score
- Fetch the actual documents
