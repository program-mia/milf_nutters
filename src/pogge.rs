mod filere;
mod fetche;

use error_chain::error_chain;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// logic for the app itself, like generating things etc.
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub struct Nutter {
    pub library: String,
    pub library_urls: Vec<String>,
}

impl Nutter {
    pub fn init(library: String) -> Nutter {
        return Nutter {
            library: library,
            library_urls: vec!{},
        };
    }

    pub fn load_library_resources(&mut self) -> &Nutter {
        self.library_urls = filere::load_library_urls(&self.library);

        if self.library_urls.len() == 0 {
            println!("No urls found in the specified library. You need to select a different library or add urls to the selected library.");

            return self;
        }

        for url in &self.library_urls {
            let filename = Nutter::hash_string(url.clone());

            if filere::is_resource_file_available(filename.to_string()) {
                continue; 
            }

            let mut entities: Vec<String> = fetche::loads_words_array_from_url(url.clone());

            match filere::create_new_resource_file_with_data(filename.to_string(), &mut entities) {
                Err(error) => println!("Failed to save data for {} into a file: {}", url.clone(), error),
                _ => {},
            };
        }

        return self;
    }

    pub fn build_entities_graph(&self) -> &Nutter {
       // TODO
        for url in &self.library_urls {
            let filename = Nutter::hash_string(url.clone());

            if ! filere::is_resource_file_available(filename.to_string()) {
                continue; 
            }

            let entities: Vec<String> = filere::load_resource_file_words_into_vector(filename.to_string()); 
            let mut prev_entity: String = String::new();

            for mut entity in entities {
                entity.make_ascii_lowercase();

                self.add_entity_to_graph_with_previous_connection(entity.clone(), prev_entity.clone());

                prev_entity = entity;
            }
        }

        return self;
    }

    fn add_entity_to_graph_with_previous_connection(&self, entity: String, prev_entity: String) {
        // TODO add entity to a graph if it doesn't exist yet (omit if empty string)
        // when adding to a graph, remember to also add entry in the hash map so that it's easeir
        // to find
        // when prev_entity is not empty string, update the link from prev to new entity with count 
    }

    fn hash_string(string: String) -> u64 {
        let mut hasher = DefaultHasher::new();

        string.hash(&mut hasher);

        return hasher.finish();
    }
}
