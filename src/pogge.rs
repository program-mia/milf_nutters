mod filere;
mod fetche;

use error_chain::error_chain;
use self::fetche::loads_words_array_from_url;

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
// TODO create a struct here that will hold/run/do everything and it should all be run on that
// structure

impl Nutter {
    pub fn init(library: String) -> Nutter {
        return Nutter {
            library: library,
        };
    }

    // TODO create stuff here related to loading stuff, etc. etc.
    pub fn load_library(&self) -> &Nutter {
        let library_urls: Vec<String> = filere::load_library_urls(&self.library);

        if library_urls.len() == 0 {
            println!("No urls found in the specified library. You need to select a different library or add urls to the selected library.");

            return self;
        }

        // the whole URL load should probably happen in a separate method, etc. etc.
        // it probably should just load data from urls and save them if needed
        for url in library_urls {
            let entities: Vec<String> = loads_words_array_from_url(url);

            for entity in entities {
                println!("{} ", entity);
            }

            //if there is no file with a given name, idk how im naming them yet but we'll see, maybe
            //csv or something, then save it all in the file, etc. 
        }

        // after the above, scan all files and load stuff and start building the whole graph

        return self;
    }
}
