use error_chain::error_chain;
use std::io::Read;
use regex::Regex;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("https://www.gutenberg.org/cache/epub/345/pg345-images.html")?;

    if ! res.status().is_success() {
        println!("Failed to fetch data from the URL. Status: {}", res.status());

        return Ok(());
    }

    let mut body: String = String::new();
    res.read_to_string(&mut body)?;

    let paragraph_regex = Regex::new(r"<p>(.|\n)*?</p>").unwrap();
    let tags_regex = Regex::new(r"<[\s\S]*?>").unwrap();
    let single_word_or_entity_regex = Regex::new(r"[a-zA-Z'’]+|[^A-Za-z'’\s]{1}").unwrap();

    for captured in paragraph_regex.captures_iter(&body) {
        let paragraph = &captured[0];
        let clear_paragraph = tags_regex.replace_all(paragraph, "");

        for word_or_entity in single_word_or_entity_regex.captures_iter(&clear_paragraph) {
            println!("Word or entity: {}\n", &word_or_entity[0]);
        }
    }

    //println!("Status {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);

    return Ok(());
}
