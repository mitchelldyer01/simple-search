use crate::Corpus;

trait Filter {
    fn tokenize(&self) -> Vec<&str>;
    fn to_lowercase(&self) -> Vec<&str>;
    fn remove_punctuation(&self) -> Vec<&str>;
    fn remove_stopwords(&self) -> Vec<&str>;
    // fn apply_stemming(&self) -> Self;
    // fn parse_text(&self) -> Self;
    // fn count_frequencies(&self) -> Vec<TermPerDocument> {}
}

struct CorpusFilter<'a> {
    document: &'a dyn Corpus,
    tokens: Vec<&'a str>,
}

impl Filter for CorpusFilter<'_> {
    fn tokenize(&self) -> Vec<&str> {
        self.document.get_body().split(" ").collect()
    }

    fn to_lowercase(&self) -> Vec<&str> {
        self.tokens
            .iter_mut()
            .for_each(
                move |token| {
                    token = &mut token.to_lowercase().as_str()
                });

        self.tokens
    }

    fn remove_punctuation(&self) -> Vec<&str> {
        self.tokens
            .iter_mut()
            .for_each(
                |token| {
                    token
                    .chars()
                    .filter(|c| is_punctuation(c))
                    .for_each(
                        |c| {
                            token.replace(c, "");
                        }
                    )
                }
            );

        self.tokens
    }

    fn remove_stopwords(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .filter(|token| !is_stopword(token) )
            .map(|token| token.as_ref() )
            .collect()
            
    }
}

struct QueryFilter {
    query: String,
}

impl Filter for QueryFilter {}

fn is_punctuation(c: &char) -> bool {
    c.eq(&'!') || c.eq(&'\\') || c.eq(&'#') || c.eq(&'$') || c.eq(&'%') || c.eq(&'&') || c.eq(&'/') || c.eq(&'(') || c.eq(&')') || c.eq(&'*') || c.eq(&'+') || c.eq(&'-') || c.eq(&',') || c.eq(&'.') || c.eq(&':') || c.eq(&';') || c.eq(&'<') || c.eq(&'=') || c.eq(&'>') || c.eq(&'?') || c.eq(&'@') || c.eq(&'[') || c.eq(&']') || c.eq(&'^') || c.eq(&'`') || c.eq(&'{') || c.eq(&'}') || c.eq(&'|')
}

fn is_stopword(token: &str) -> bool {
    token.eq("the") || token.eq("be") || token.eq("to") || token.eq("of") || token.eq("and") || token.eq("a") || token.eq("in") || token.eq("that") || token.eq("have") || token.eq("i") || token.eq("it") || token.eq("for") || token.eq("not") || token.eq("on") || token.eq("with") || token.eq("he") || token.eq("as") || token.eq("you") || token.eq("do") || token.eq("at") || token.eq("this") || token.eq("but") || token.eq("his") || token.eq("by") || token.eq("from")
}