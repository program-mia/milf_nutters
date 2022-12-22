mod filere;
mod fetche;

use error_chain::error_chain;
use self::fetche::{url_array, loads_words_array_from_url};

// logic for the app itself, like generating things etc.
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn run() {
    // the whole URL load should probably happen in a separate method, etc. etc.
    // it probably should just load data from urls and save them if needed
    for url in url_array() {
        let entities: Vec<String> = loads_words_array_from_url(url);

        for entity in entities {
            println!("{} ", entity);
        }

        //if there is no file with a given name, idk how im naming them yet but we'll see, maybe
        //csv or something, then save it all in the file, etc. 
    }

    // after the above, scan all files and load stuff and start building the whole graph
}
