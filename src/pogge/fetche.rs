//this one will be responsible for loading words from external sources if needed, saving it to
//files and stuff
use std::io::Read;
use regex::Regex;
use reqwest::{blocking::Response, Error};

pub fn url_array() -> [String; 1] {
    // TODO probably could load it form a file and stuff
    return [
        "https://www.gutenberg.org/cache/epub/345/pg345-images.html".to_string(),
    ];
}

fn load_data_from_url(url: String) -> Result<Response, Error> {
    let response = reqwest::blocking::get(url)?;

    if ! response.status().is_success() {
        println!("failed to fetch data from the url - {}. status: {}", response.url(), response.status());
    }

    return Ok(response);
}

fn transform_get_response_to_string(mut response: Response) -> Result<String, std::io::Error> {
    let mut body: String = String::new();

    response.read_to_string(&mut body)?;

    return Ok(body);
}

pub fn loads_words_array_from_url(url: String) -> Vec<String> {
    let url_data: Response = match load_data_from_url(url) {
        Ok(result) => result,
        Err(_) => return Vec::new(),
    };

    let string_data: String = match transform_get_response_to_string(url_data) {
        Ok(result) => result,
        Err(_) => return Vec::new(),
    };

    let paragraph_regex = Regex::new(r"<p>(.|\n)*?</p>").unwrap();
    let tags_regex = Regex::new(r"<[\s\S]*?>").unwrap();
    let single_word_or_entity_regex = Regex::new(r"[a-zA-Z'’]+|[^a-zA-Z'’\s]{1}").unwrap();

    for captured in paragraph_regex.captures_iter(&string_data) {
        let paragraph = &captured[0];
        let clear_paragraph = tags_regex.replace_all(paragraph, "");

        for word_or_entity in single_word_or_entity_regex.captures_iter(&clear_paragraph) {
            println!("word or entity: {}\n", &word_or_entity[0]);
        }
    }

    return Vec::new();
}
