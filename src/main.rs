use std::{env, io::BufRead, io::BufReader};
use std::fs::File;
use rust_stemmers::{Algorithm, Stemmer};
use corpus::{PlainText};
use filter::{Filter, is_stopword};
use indexer::InvertedIndex;
mod corpus;
mod filter;
mod indexer;

fn main() {
    let path_to_list: String = env::var("PATH_TO_LIST")
        .unwrap_or(String::from("list"));

    let mut corpi: Vec<PlainText> = Vec::new();

    let list = File::open(path_to_list);
    match list {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(text) => {
                        let corpus = PlainText::new(text, corpi.len().try_into().unwrap());
                        match corpus {
                            Ok(c) => corpi.push(c),
                            Err(err) => eprintln!("{}", err),
                        }
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let stemmer = Stemmer::create(Algorithm::English);
    let filter = Filter{};
    let mut index = InvertedIndex::new();

    corpi
        .iter()
        .for_each(
            |corpus|{
                corpus.get_body()
                    .iter()
                    .for_each(|word| {
                        if !is_stopword(word) {
                            index.add_document(
                                filter.clean(word.to_string(), &stemmer), 
                                corpus.get_title().to_string());
                        }
                    }
                )
            }
        );
}
