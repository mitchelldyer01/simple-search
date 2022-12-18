use rust_stemmers::{Algorithm, Stemmer};
use regex::Regex;
use crate::Corpus;

const STOPWORDS: [&'static str; 25] = ["the", "be", "to", "of", "and",
                                    "a", "in", "that", "have", "i",
                                    "it", "for", "not", "on", "with",
                                    "he", "as", "you", "do", "at",
                                    "this", "but", "his", "by", "from"];

trait Filter {
    fn tokenize(body: String) -> Vec<String> {
        body.split_whitespace().map(str::to_string).collect()
    }

    fn to_lowercase(tokens: Vec<String>) -> Vec<String> {
        tokens
            .iter()
            .map(
                |token| {
                    token.to_lowercase().to_string()
                }
            )
            .collect()
    }

    fn remove_punctuation(tokens: Vec<String>) -> Vec<String> {
        tokens
            .iter()
            .map(
                |token| {
                    replace_punctuation(token, "")
                }
            )
            .collect()
    }

    fn remove_stopwords(tokens: Vec<String>) -> Vec<String> {
        tokens
            .iter()
            .filter(
                |token| !is_stopword(*token)
            )
            .cloned()
            .collect()
    }

    fn apply_stemming(tokens: Vec<String>) -> Vec<String> {
        let en_stemmer = Stemmer::create(Algorithm::English);

        tokens
            .iter()
            .map(
                |token| {
                    token
                        .replace(
                            token,
                            en_stemmer
                                .stem(token)
                                .to_string()
                                .as_str()
                        )
                }
            )
            .collect()
    }
    // fn parse_text(&self) -> Self;
    // fn count_frequencies(&self) -> Vec<TermPerDocument> {}
}

struct CorpusFilter<'a> {
    document: &'a dyn Corpus,
    tokens: Vec<String>,
}

impl Filter for CorpusFilter<'_> {}

struct QueryFilter {
    query: String,
    tokens: Vec<String>,
}

impl Filter for QueryFilter {}

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
    STOPWORDS.iter().any(|stopword| token.to_string().eq(stopword))
}

#[test]
fn test_is_stopword() {
    assert_eq!(is_stopword("the"), true);
    assert_eq!(is_stopword("plato"), false);
}

#[test]
fn test_replace_punctuation() {
    assert_eq!(replace_punctuation("dog", ""), "dog");
    assert_eq!(replace_punctuation("dog.,/\\<>!@#$%^&*()-=+[]{}|", ""), "dog");
}