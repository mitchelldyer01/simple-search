use reqwest::Error as ReqError;

pub trait Corpus {
    fn get_id(&self) -> &u32;
    fn get_title(&self) -> &String;
    fn get_author(&self) -> &String;
    fn get_body(&self) -> &String;
}

pub struct PlainText {
    id: u32,
    title: String,
    body: String,
    author: String,
}

impl Corpus for PlainText{
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn get_author(&self) -> &String {
        &self.author
    }

    fn get_body(&self) -> &String {
        &self.body
    }
}

impl PlainText {
    pub fn new(path: String, id: u32) -> Result<Self, ReqError> {
        let text: String;
        let response = reqwest::blocking::get(path);

        match response {
            Ok(res) => text = res.text().unwrap(),
            Err(err) => return Err(err),
        }
    

        Ok(Self{
            id: id,
            title: Self::set_title(&text),
            author: Self::set_author(&text),
            body: text,
        }) 
    }

    fn set_title(text: &String) -> String {
        Self::search_for_line(
            text.to_string(), 
            String::from("Title: ")
        )
        .replace("Title: ", "")
    }

    fn set_author(text: &String) -> String {
        Self::search_for_line(
            text.to_string(),
            String::from("Author: ")
        )
        .replace("Author: ", "")
    }

    fn search_for_line(text: String, query: String) -> String {
        text
            .lines()
            .filter(|line| line.contains(&query))
            .collect()
    }
}
