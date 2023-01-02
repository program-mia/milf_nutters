use std::fs::File;
use std::io::prelude::*;
use json::{JsonValue, array};

fn get_project_root_path() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();

    path.pop();

    return path;
}

fn get_file_contents_as_string(filename: String) -> std::io::Result<String> {
    let mut file = File::open(format!("{}/{}", get_project_root_path().display(), filename))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

pub fn load_library_urls(library: &String) -> Vec<String> {
    let file_contents = match get_file_contents_as_string("libraries.json".to_string()) {
        Ok(contents) => contents,
        Err(error) => {
            println!("There was an error while loading the libraries file.\n{error}");

            "{}".to_string()
        }
    };

   let mut parsed_content: JsonValue = match json::parse(&file_contents) {
        Ok(result) => result,
        Err(error) => {
            println!("Failed to parse JSON data.\n{error}");

            array![]
        }
    };

    if parsed_content.is_empty() || ! parsed_content.has_key(&library)  {
        println!("Libraries file is empty or the selected library was not found in the libraries file.");

        return vec!{};
    }

    let mut return_data = Vec::new();

    for (key,urls_array) in parsed_content.entries_mut() {
        if key != library {
            continue;
        }

        while urls_array.len() > 0 {
            let url = urls_array.pop();

            return_data.push(url.to_string());
        }
    }

    return return_data;
}
