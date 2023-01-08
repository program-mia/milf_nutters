use std::path::PathBuf;
use std::fs::{File, create_dir};
use std::io::prelude::*;
use json::{JsonValue, array};

pub fn create_new_resource_file_with_data(filename: String, entities: &mut Vec<String>) -> std::io::Result<()> {
    create_resources_directory_if_needed();

    let mut file = File::create(get_full_path_for_resource_file(filename).as_path())?;

    for entity in entities.iter_mut() {
        if *entity == "\n" {
            *entity = "\\n".to_string();
        }
    }

    file.write_all(entities.join("\n").as_bytes())?;

    return Ok(());
}

pub fn load_resource_file_words_into_vector(filename: String) -> Vec<String> {
    let path = get_full_path_for_resource_file(filename);
    let mut entities: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(String::from(path.to_str().unwrap())) {
        for line in lines {
            if let Ok(entity) = line {
                let corrected_entity_value: String = if entity == "\\n" {
                    "\n".to_string()
                } else {
                    entity
                };

                entities.push(corrected_entity_value);
            }
        }
    }

    return entities;
}

fn read_lines(filename: String) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(std::io::BufReader::new(file).lines())
}

pub fn is_resource_file_available(filename: String) -> bool {
    let path = get_full_path_for_resource_file(filename);

    return path.as_path()
        .exists();
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

    for (key, urls_array) in parsed_content.entries_mut() {
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

fn get_full_path_for_resource_file(filename: String) -> PathBuf{
    let mut path = get_project_root_path();

    path.push("resources");
    path.push(filename);
    path.set_extension("txt");

    return path;
}

fn create_resources_directory_if_needed() {
    let mut path = get_project_root_path();

    path.push("resources");

    if path.exists() {
        return;
    }

    match create_dir(path) {
        Err(error) => println!("Failed to create resources directory: {}", error),
        Ok(_) => {},
    };
}

pub fn get_libraries_file_as_json() -> JsonValue {
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

    if parsed_content.is_empty() {
        println!("Libraries file is empty or the selected library was not found in the libraries file.");
    }

    return parsed_content;
}
