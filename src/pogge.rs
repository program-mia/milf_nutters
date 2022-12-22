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
    for url in url_array() {
        loads_words_array_from_url(url);
    }
}
