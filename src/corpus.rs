use reqwest::Error as ReqError;

#[derive(Debug)]
pub struct PlainText {
    id: u32,
    title: String,
    body: Vec<String>,
    author: String,
}

impl PlainText{
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_author(&self) -> &String {
        &self.author
    }

    pub fn get_body(&self) -> &Vec<String> {
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
            body: Self::set_body(&text),
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

    fn set_body(text: &String) -> Vec<String> {
        text.split_whitespace().map(str::to_string).collect()
    }

    fn search_for_line(text: String, query: String) -> String {
        text
            .lines()
            .filter(|line| line.contains(&query))
            .collect()
    }
}
