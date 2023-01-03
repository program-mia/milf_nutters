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
}

impl Nutter {
    pub fn init(library: String) -> Nutter {
        return Nutter {
            library: library,
        };
    }

    pub fn load_library_resources(&self) -> &Nutter {
        let library_urls: Vec<String> = filere::load_library_urls(&self.library);

        if library_urls.len() == 0 {
            println!("No urls found in the specified library. You need to select a different library or add urls to the selected library.");

            return self;
        }

        for url in library_urls {
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

    // TODO create a method to build a connection graph for the selected library 

    fn hash_string(string: String) -> u64 {
        let mut hasher = DefaultHasher::new();

        string.hash(&mut hasher);

        return hasher.finish();
    }
}
