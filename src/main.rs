use std::{env, io::BufRead, io::BufReader};
use std::fs::File;
use corpus::{PlainText};
use crate::filter::{CorpusFilter};
mod corpus;
mod filter;

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

    let mut corpi_filters: Vec<CorpusFilter> = Vec::new();

    corpi
        .iter()
        .for_each(
            |corpus|{
                let filter = CorpusFilter::new(corpus);
                corpi_filters.push(filter);
            }
        );
    
}
