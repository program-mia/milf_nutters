mod filere;
mod fetche;

use error_chain::error_chain;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;
use rand::{thread_rng, Rng};

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
    pub graph: HashMap<String, Node>,
    pub is_library_loaded: bool,
}

pub struct Node {
    pub entity: String,
    pub connections: Vec<NodeConnection>,
    pub total_connections_amount: u32,
}

pub struct NodeConnection {
    pub occurences: u32, 
    pub entity: String,
}

pub struct Library {
    pub name: String,
    pub urls: Vec<String>,
}

impl Nutter {
    pub fn init(library: String) -> Nutter {
        return Nutter {
            library: library,
            library_urls: vec!{},
            graph: HashMap::new(),
            is_library_loaded: false,
        };
    }

    pub fn load_library_resources(&mut self, needs_full_refetch: bool) -> &mut Nutter {
        let timer = Instant::now();

        self.library_urls = filere::load_library_urls(&self.library);

        if self.library_urls.len() == 0 {
            println!("No urls found in the specified library. You need to select a different library or add urls to the selected library.");

            return self;
        }

        for url in self.library_urls.iter() {
            let filename = Nutter::hash_string(url.clone());

            if filere::is_resource_file_available(filename.to_string()) && ! needs_full_refetch {
                continue; 
            }

            let mut entities: Vec<String> = fetche::loads_words_array_from_url(url.clone());

            match filere::create_new_resource_file_with_data(filename.to_string(), &mut entities) {
                Err(error) => println!("Failed to save data for {} into a file: {}", url.clone(), error),
                _ => {},
            };
        }

        println!("Library resources loaded in {:?}", timer.elapsed());

        return self;
    }

    pub fn build_entities_graph(&mut self) -> &mut Nutter {
        let full_function_timer = Instant::now();
        // TODO this is overall slow, need to speed this up
        let mut urls: Vec<String> = vec!{};

        // Construct a local vector of URLs to not have a reference mismatch later
        for url in self.library_urls.iter() {
            urls.push(url.to_string());
        }

        for url in urls.iter() {
            let single_url_timer = Instant::now();
            let filename = Nutter::hash_string(url.clone());

            if ! filere::is_resource_file_available(filename.to_string()) {
                continue; 
            }

            let entities: Vec<String> = filere::load_resource_file_words_into_vector(filename.to_string()); 

            let mut prev_entity: String = "".to_string();

            for mut entity in entities {
                entity.make_ascii_lowercase();

                // feed it in reverse entity order to make graph actually work
                self.add_entity_to_graph_with_previous_connection(prev_entity.clone(), entity.clone());

                prev_entity = entity;
            }

            match self.graph.get_mut(&prev_entity) {
                Some(graph_entity) => graph_entity.connections.push(NodeConnection {
                    occurences: 1,
                    entity: "".to_string(),
                }),
                None => println!("Error while adding graph entity: {}", prev_entity),
            };

            println!("Graph built for url \"{}\" in {:?}", url, single_url_timer.elapsed());
        }

        println!("Words graph built in {:?}", full_function_timer.elapsed());

        self.is_library_loaded = true;

        return self;
    }

    fn add_entity_to_graph_with_previous_connection(&mut self, entity: String, prev_entity: String) {
        if entity.is_empty() {
            return;
        }

        self.graph.entry(entity.to_string()).or_insert(Node {
            entity: entity.clone(),
            connections: vec!{},
            total_connections_amount: 0,
        });

        if prev_entity.is_empty() {
            return;
        }

        let mut found_connection_index = 0;
        
        // overall this loop is going to perform really bad when building stuff, need to somehow
        // fix it
        for index in 0..self.graph.get(&entity).unwrap().connections.len() {
            if self.graph.get(&entity).unwrap().connections[index].entity != prev_entity {
                continue;
            }
            
            self.graph.get_mut(&entity)
                .unwrap()
                .connections[index]
                .occurences += 1;        

            found_connection_index = index;

            self.graph.get_mut(&entity).unwrap().total_connections_amount += 1;
            
            break;
        }

        if found_connection_index != 0 {
            return;
        }

        self.graph.get_mut(&entity).unwrap().connections.push(NodeConnection {
            occurences: 1,
            entity: prev_entity.clone(),
        });

        self.graph.get_mut(&entity).unwrap().total_connections_amount += 1;
    }

    // Note that this only works for console
    pub fn print_graph(&mut self) -> &mut Nutter {
        if ! self.is_library_loaded {
            println!("Graph is not loaded, you must first build the graph.");
        }

        for (_, node) in self.graph.iter() {
            println!("Node: {} - # of connections - {} with {} in total", node.entity, node.connections.len(), node.total_connections_amount);
        }

        return self;
    }

    //TODO add method to clear resources file, so that you can re-fetch it
    //TODO add method to add something to a given library array in JSON
    //TODO add method to return all urls in resource 
    //TODO add method to return all available resource groups
    //TODO add method to return array/vector of the graph (similar to the print method)
    //TODO add method to print X sentences

    // TODO maybe return last entity so that I can see if maybe the end was met or something.
    pub fn print_sentence_starting_from(&self, mut entity: String) -> &Nutter {
        // If entity is empty, find a starting point
        let word_regex = Regex::new("[a-zA-Z]+").unwrap(); 

        if entity.is_empty() {
            let mut rng = thread_rng();

            while ! word_regex.is_match(&entity) {
                let key_index = rng.gen_range(0..self.graph.len()); 
                let mut index = 0;
                    
                for (_, node) in self.graph.iter() {
                    if index == key_index {
                       entity = node.entity.clone(); 

                        break;
                    }

                    index += 1;
                }
            }
        }

        let mut is_new_sentence = true;

        while ! entity.is_empty() && entity != "" && entity != "." {
            if is_new_sentence {
                entity = Nutter::uppercase_first_string_letter(entity);
            }

            if word_regex.is_match(&entity) && ! is_new_sentence {
                print!(" ");
            }

            print!("{}", entity);

            entity = self.get_next_entity_after(entity);
            is_new_sentence = false;
        }
        
        print!("{}", entity);
        
        return self;
    }

    fn get_next_entity_after(&self, mut entity: String) -> String {
        entity = entity.to_lowercase();

        if ! self.graph.contains_key(&entity) {
            println!("did not find {} in graph", entity);

            return String::new();
        }

        // TODO it would be nice to maybe increase the probability of hitting a sentence ending
        // character with the length increasing
        
        let mut rng = thread_rng();
        let selected_index = rng.gen_range(0..self.graph.get(&entity).unwrap().total_connections_amount);
        let mut current_index = 0;

        for connection in self.graph.get(&entity).unwrap().connections.iter() {
            if current_index <= selected_index && selected_index <= current_index + connection.occurences {
                entity = connection.entity.clone();

                break;
            }

            current_index += connection.occurences;
        }

        return match self.graph.get(&entity).unwrap().total_connections_amount {
            0 => String::new(),
            _ => self.graph.get(&entity).unwrap().entity.clone(),
        };
    }

    pub fn get_libraries_list(&self) -> Vec<Library> {
        let mut library_data = filere::get_libraries_file_as_json();
        let mut parsed_data: Vec<Library> = vec!{};

        for (key, urls) in library_data.entries_mut() {
            let mut urls_vec: Vec<String> = vec!{}; 

            for url in urls.members() {
                urls_vec.push(url.clone().to_string());
            }

            parsed_data.push(Library {
                name: key.to_string(),
                urls: urls_vec,
            });
        }
        
        return parsed_data;
    }

    pub fn remove_url_from_library(&mut self, url: String, library_name: String) -> std::result::Result<(), String> {
        // TODO I should validate if the URL is actually real, etc.
        if url.is_empty() {
            return Err("URL is required".to_string());
        }

        if library_name.is_empty() {
            return Err("Library name is required.".to_string());
        }

        let libraries: Vec<Library> = self.get_libraries_list();
        let mut is_library_in_list: bool = false;

        for library in libraries.iter() {
            if library.name == library_name {
                is_library_in_list = true;
            }
        }

        if ! is_library_in_list {
            return Err("Library does not exist.".to_string());
        }

        return filere::remove_url_from_library(url, library_name);    
    }

    pub fn add_url(&mut self, url: String, library_name: String) -> std::result::Result<(), String> {
        // TODO I should validate if the URL is actually real, etc.
        if url.is_empty() {
            return Err("URL is required".to_string());
        }

        if library_name.is_empty() {
            return Err("Library name is required.".to_string());
        }

        let libraries: Vec<Library> = self.get_libraries_list();
        let mut is_library_in_list: bool = false;

        for library in libraries.iter() {
            if library.name == library_name {
                is_library_in_list = true;
            }
        }

        if ! is_library_in_list {
            return Err("Library does not exist.".to_string());
        }

        return filere::add_url_to_library(url, library_name);    
    }

    pub fn add_library(&mut self, library_name: String) -> std::result::Result<(), String> {
        if library_name.is_empty() {
            return Err("Library name is required.".to_string());
        }

        let libraries = self.get_libraries_list();

        for library in libraries.iter() {
            if library.name == library_name {
                return Err("Library with that name already exists.".to_string());
            }
        }

        if libraries.len() >= 20 {
            return Err("You cannot have more than 20 libraries.".to_string());
        }

        return filere::add_library(library_name);    
    }

    pub fn remove_library(&mut self, library_name: String) -> std::result::Result<(), String> {
        if library_name.is_empty() {
            return Err("Library name is required.".to_string());
        }

        if library_name == "default" {
            return Err("You cannot delete the default library.".to_string());
        }

        let libraries: Vec<Library> = self.get_libraries_list();
        let mut is_library_in_list: bool = false;

        for library in libraries.iter() {
            if library.name == library_name {
                is_library_in_list = true;
            }
        }

        if ! is_library_in_list {
            return Err("This library is not in the list of libraries.".to_string());
        }

        return filere::remove_library(library_name);    
    }
    
    pub fn set_library(&mut self, library_name: String) -> std::result::Result<(), String> {
        let libraries: Vec<Library> = self.get_libraries_list(); 
        let mut is_library_present = false;

        for library in libraries {
            if library.name == library_name {
                is_library_present = true;

                break;
            }
        }

        if ! is_library_present {
            return Err("This library does not exist.".to_string());
        }

        self.is_library_loaded = library_name == self.library;
        self.library = library_name.clone();

        return Ok(());
    }

    fn uppercase_first_string_letter(string: String) -> String {
        let mut character = string.chars();

        return match character.next() {
            None => String::new(),
            Some(letter) => letter.to_uppercase().collect::<String>() + character.as_str(),
        };
    }

    fn hash_string(string: String) -> u64 {
        let mut hasher = DefaultHasher::new();

        string.hash(&mut hasher);

        return hasher.finish();
    }
}
