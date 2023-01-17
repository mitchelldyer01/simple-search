use rust_stemmers::{Stemmer};
use regex::Regex;

const STOPWORDS: [&'static str; 25] = ["the", "be", "to", "of", "and",
                                    "a", "in", "that", "have", "i",
                                    "it", "for", "not", "on", "with",
                                    "he", "as", "you", "do", "at",
                                    "this", "but", "his", "by", "from"];

pub struct Filter {}

impl Filter {
    pub fn clean(&self, token: String, stemmer: &Stemmer) -> String {
        let mut filtered = token;
        filtered = filtered.to_lowercase();
        filtered = replace_punctuation(&filtered, "");
        filtered = filtered.replace(&filtered, stemmer.stem(&filtered).to_string().as_str());
        filtered
    }
}

fn replace_punctuation(word: &str, replacement: &str) -> String {
    /*
    [   Character block start.
    ^   Not these characters.
    \w  Word characters.
    \s  Space characters.
    ]   Character block end.
     */
    let regexer = Regex::new(r"[^\w\s]").unwrap();
    regexer.replace_all(word, replacement).to_string()
}

pub fn is_stopword(token: &str) -> bool {
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

#[test]
fn test_clean() {
    const TEST_TOKENS: [&'static str; 5] = ["TEST", "CASE!", "PLEASE", "IGNORE,", "THANKFULLY"];
    const FILTERED_TEST_QUERY: [&'static str; 5] = ["test", "case", "pleas", "ignor", "thank"];

    let f = Filter{};
    let en_stemmer = Stemmer::create(rust_stemmers::Algorithm::English);

    for i in 0..FILTERED_TEST_QUERY.len() {
        assert_eq!(f.clean(TEST_TOKENS[i].to_string(), &en_stemmer), FILTERED_TEST_QUERY[i]);
    }
}