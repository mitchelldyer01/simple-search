use crate::corpus::{Corpus};
use rust_stemmers::{Algorithm, Stemmer};
use regex::Regex;

const STOPWORDS: [&'static str; 25] = ["the", "be", "to", "of", "and",
                                    "a", "in", "that", "have", "i",
                                    "it", "for", "not", "on", "with",
                                    "he", "as", "you", "do", "at",
                                    "this", "but", "his", "by", "from"];

trait Filter {
    fn tokenize(body: String) -> Vec<String> {
        body.split_whitespace().map(str::to_string).collect()
    }

    fn clean(&mut self) {}

    fn to_lowercase(&mut self) {}

    fn remove_punctuation(&mut self) {}

    fn remove_stopwords(&mut self) {}

    fn apply_stemming(&mut self) {}
    // fn parse_text(&self) -> Self;
    // fn count_frequencies(&self) -> Vec<TermPerDocument> {}
}

pub struct CorpusFilter {
    tokens: Vec<String>,
}

impl Filter for CorpusFilter {
    fn clean(&mut self) {
        let en_stemmer = Stemmer::create(Algorithm::English);

        let clean_iter = self.tokens.iter()
            .map(
                |token| {
                    token.to_lowercase().to_string();
                    replace_punctuation(token, "");
                    token.replace(token,
                            en_stemmer
                                .stem(token)
                                .to_string()
                                .as_str()
                    )
                }
            );

        let stripped_iter: Vec<String> = clean_iter.collect();

        let filtered_iter = stripped_iter
            .into_iter()
            .filter(
                |token| !is_stopword(token)
        );

        self.tokens = filtered_iter.collect();
    }

}
impl CorpusFilter {
    pub fn new(doc: & dyn Corpus) -> CorpusFilter {
        let mut filter = Self{
            tokens: Self::tokenize(doc.get_body().to_string()),
        };

        filter.clean();

        filter
    }
}

struct QueryFilter {
    query: String,
    tokens: Vec<String>,
}

impl Filter for QueryFilter {}
impl QueryFilter {
    fn new(q: &str) -> QueryFilter {
        let query = q.to_string();
        
        let mut filter = Self { 
            query: query.clone(), 
            tokens: Self::tokenize(query.clone()) 
        };

        filter.to_lowercase();
        filter.remove_punctuation();
        filter.remove_stopwords();
        filter.apply_stemming();

        filter
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

#[test]
fn test_filters() {
    const TEST_QUERY: &str = "TEST CASE! PLEASE IGNORE IT, THANKFULLY";
    const FILTERED_TEST_QUERY: [&'static str; 5] = ["test", "case", "pleas", "ignor", "thank"];

    let query = QueryFilter::new(TEST_QUERY);
    assert_eq!(query.tokens, FILTERED_TEST_QUERY);
}