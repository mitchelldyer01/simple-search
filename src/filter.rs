use rust_stemmers::{Algorithm, Stemmer};
use regex::Regex;
use crate::Corpus;

trait Filter {
    fn tokenize(&self) -> Vec<String>;
    fn to_lowercase(self);
    fn remove_punctuation(self);
    fn remove_stopwords(self);
    fn apply_stemming(self);
    // fn parse_text(&self) -> Self;
    // fn count_frequencies(&self) -> Vec<TermPerDocument> {}
}

struct CorpusFilter<'a> {
    document: &'a dyn Corpus,
    tokens: Vec<String>,
}

impl Filter for CorpusFilter<'_> {
    fn tokenize(&self) -> Vec<String> {
        self.document.get_body().split_whitespace().map(str::to_string).collect()
    }

    fn to_lowercase(mut self) {
        self.tokens = self.tokens
            .iter()
            .map(
                |token| {
                    token.to_lowercase().to_string()
                }
            )
            .collect();
    }

    fn remove_punctuation(mut self) {
        self.tokens = self.tokens
            .iter()
            .map(
                |token| {
                    replace_punctuation(token, "")
                }
            )
            .collect();
    }

    fn remove_stopwords(mut self) {
        self.tokens = self.tokens.iter().filter(|token| !is_stopword(*token)).cloned().collect();
    }

    fn apply_stemming(mut self) {
        let en_stemmer = Stemmer::create(Algorithm::English);

        self.tokens = self.tokens
            .iter()
            .map(
                |token| {
                    token.replace(token, en_stemmer.stem(token).to_string().as_str())
                }
            )
            .collect();
    }
}

struct QueryFilter {
    query: String,
}

// impl Filter for QueryFilter {}

fn replace_punctuation(word: &str, replacement: &str) -> String {
    /*
    [   Character block start.
    ^   Not these characters (letters, numbers).
    \w  Word characters.
    \s  Space characters.
    ]   Character block end.
     */
    let regexer = Regex::new(r"[^\w\s]").unwrap();
    regexer.replace_all(word, replacement).to_string()
}

fn is_stopword(token: &str) -> bool {
    token.eq("the") || token.eq("be") || token.eq("to") || token.eq("of") || token.eq("and") || token.eq("a") || token.eq("in") || token.eq("that") || token.eq("have") || token.eq("i") || token.eq("it") || token.eq("for") || token.eq("not") || token.eq("on") || token.eq("with") || token.eq("he") || token.eq("as") || token.eq("you") || token.eq("do") || token.eq("at") || token.eq("this") || token.eq("but") || token.eq("his") || token.eq("by") || token.eq("from")
}