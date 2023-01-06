mod filere;
mod fetche;

use error_chain::error_chain;
use std::collections::HashMap;
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
    pub graph: HashMap<String, Node>,
}

pub struct Node {
    pub entity: String,
    pub connections: Vec<NodeConnection>,
}

pub struct NodeConnection {
    pub occurences: u32, 
    pub entity: String,
}

impl Nutter {
    pub fn init(library: String) -> Nutter {
        return Nutter {
            library: library,
            library_urls: vec!{},
            graph: HashMap::new(),
        };
    }

    pub fn load_library_resources(&mut self) -> &mut Nutter {
        self.library_urls = filere::load_library_urls(&self.library);

        if self.library_urls.len() == 0 {
            println!("No urls found in the specified library. You need to select a different library or add urls to the selected library.");

            return self;
        }

        for url in self.library_urls.iter() {
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

    pub fn build_entities_graph(&mut self) -> &mut Nutter {
        let mut urls: Vec<String> = vec!{};

        // Construct a local vector of URLs to not have a reference mismatch later
        for url in self.library_urls.iter() {
            urls.push(url.to_string());
        }

        for url in urls.iter() {
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

            self.graph.get_mut(&prev_entity).unwrap().connections.push(NodeConnection {
                occurences: 1,
                entity: "".to_string(),
            });
        }

        return self;
    }

    fn add_entity_to_graph_with_previous_connection(&mut self, entity: String, prev_entity: String) {
        if entity.is_empty() {
            return;
        }

        self.graph.entry(entity.clone()).or_insert(Node {
            entity: entity.clone(),
            connections: vec!{},
        });

        if prev_entity.is_empty() {
            return;
        }

        let mut found_connection_index = 0;
        
        for index in 0..self.graph.get(&entity).unwrap().connections.len() {
            if self.graph.get(&entity).unwrap().connections[index].entity != prev_entity {
                continue;
            }
            
            self.graph.get_mut(&entity)
                .unwrap()
                .connections[index]
                .occurences = self.graph.get(&entity).unwrap().connections[index].occurences + 1;        

            found_connection_index = index;
        }

        if found_connection_index != 0 {
            return;
        }

        self.graph.get_mut(&entity).unwrap().connections.push(NodeConnection {
            occurences: 1,
            entity: prev_entity.clone(),
        });
    }

    pub fn print_graph(&mut self) -> &mut Nutter {
        for (_, node) in self.graph.iter() {
            println!("Node: {} - # of connections - {}", node.entity, node.connections.len());
        }

        return self;
    }

    fn hash_string(string: String) -> u64 {
        let mut hasher = DefaultHasher::new();

        string.hash(&mut hasher);

        return hasher.finish();
    }
}
