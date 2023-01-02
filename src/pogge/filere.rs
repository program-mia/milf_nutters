use std::fs::File;
use std::io::prelude::*;
use json::{JsonValue, array};
// this fileres is for managing files - loading, storing, etc. 

fn get_project_root_path() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();

    path.pop();

    return path;
}

fn get_file_contents_as_string(filename: String) -> std::io::Result<String> {
    println!("{}-{}", get_project_root_path().display(), filename);

    let mut file = File::open(format!("{}/{}", get_project_root_path().display(), filename))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

pub fn load_library_urls(library: &String) -> Vec<String> {
    // TODO open libraries file (json) and check if the given library exists. if it does, return
    let file_contents = match get_file_contents_as_string("libraries.json".to_string()) {
        Ok(contents) => contents,
        Err(error) => {
            println!("There was an error while loading the libraries file.\n{error}");

            "{}".to_string()
        }
    };

   let parsed_content: JsonValue = match json::parse(&file_contents) {
        Ok(result) => result,
        Err(error) => {
            println!("Failed to parse JSON data.\n{error}");

            array![]
        }
    };

    if parsed_content.is_empty() || ! parsed_content.has_key(&library)  {
        return vec!{};
    }

    let mut return_data = Vec::new();

    for (key, url) in parsed_content.entries() {
        println!("{key} => {url}");
        return_data.push(url.to_string());
    }

    return return_data;
}
