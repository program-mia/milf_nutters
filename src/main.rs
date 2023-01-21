mod pogge;

use std::env;
use std::sync::Mutex;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

enum RunningMode {
    Console,
    WebServer,
}

struct AppStateWithNutter {
    nutter: Mutex<pogge::Nutter>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mode = get_running_mode_from_args(args);
    
    return match mode {
        RunningMode::Console => run_in_console(),
        RunningMode::WebServer => run_as_web_server(),
    };
}

fn get_running_mode_from_args(arguments: Vec<String>) -> RunningMode { 
    let mut mode: RunningMode = RunningMode::Console; 

    for item in arguments {
        if item == "-c" || item == "--console" {
            mode = RunningMode::Console;
        }

        if item == "-s" || item == "--server" {
            mode = RunningMode::WebServer;
        }
    }

    return mode;
}

fn run_in_console() -> std::io::Result<()> {
    let console_input = std::io::stdin();
    let mut option: String = String::new();
    let library: String = "default".to_string();
    let mut loaded_status: String;

    let mut nutter = pogge::Nutter::init(library);

    while option != ":exit" {
        option = String::new();

        if nutter.is_library_loaded {
            loaded_status = "Loaded".to_string();
        } else {
            loaded_status = "Not loaded".to_string();
        }

        print_console_options(nutter.library.clone(), loaded_status.clone());

        console_input.read_line(&mut option).unwrap();
        println!();
        option.pop();

        let action = match option.clone().split_whitespace().nth(0) {
            Some(data) => data.to_string(),
            None => "".to_string(),
        };

        match action.as_str() {
            ":interactive" => use_generator_in_console(&mut nutter),
            ":set_library" => set_library_from_console(&mut nutter, option.clone()),
            ":add_library" => add_library_from_console(&mut nutter, option.clone()),
            ":remove_library" => remove_library_from_console(&mut nutter, option.clone()),
            ":print_libraries" => print_libraries(&nutter),
            ":add_url" => add_url_from_console(&mut nutter, option.clone()),
            ":remove_url" => remove_url_from_console(&mut nutter, option.clone()),
            ":fetch_data" => fetch_data_from_console(&mut nutter, option.clone()),
            ":print_graph" => print_graph(&mut nutter),
            ":build_graph" => build_graph_from_console(&mut nutter),
            ":print_sentence" => print_single_sentence_from_console(&mut nutter, option.clone()),
            ":exit" => option = action.to_string().clone(),
            _ => println!("This option does not exists, please try again."),
        };
    };


    return Ok(());
}

fn print_single_sentence_from_console(nutter: &mut pogge::Nutter, input: String) {
    let word = match input.split_whitespace().nth(1) {
        Some(result) => result.to_string(),
        None => "".to_string(),
    };

    if ! nutter.is_library_loaded {
        println!("Library was not loaded yet. You need to load libraries and load a graph first before generating sentences.");

        return;
    }

    let sentence = match nutter.get_sentence_starting_from(word) {
        Ok(answer) => answer,
        Err(error) => error,
    };
   println!("{}", sentence); 
}

fn build_graph_from_console(nutter: &mut pogge::Nutter) {
    nutter.load_library_resources(false)
        .build_entities_graph();
}

fn fetch_data_from_console(nutter: &mut pogge::Nutter, input: String) {
    let needs_full_refetch: bool = input.contains(" -c ");

    nutter.load_library_resources(needs_full_refetch);
}

fn print_graph(nutter: &mut pogge::Nutter) {
    nutter.print_graph();
}

fn print_libraries(nutter: &pogge::Nutter) {
    for library in nutter.get_libraries_list() {
        println!("Library - {}", library.name);

        for url in library.urls {
            println!("\t- {}", url);
        }
    }
}

fn remove_url_from_console(nutter: &mut pogge::Nutter, input: String) {
    let url = match input.split_whitespace().nth(1) {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    
    let library = match input.split_whitespace().nth(2) {
        Some(data) => data.to_string(),
        None => nutter.library.to_string(),
    };

    match nutter.remove_url_from_library(url, library) {
        Ok(_) => {},
        Err(error) => println!("Error occured: {}", error),
    };
}

fn add_url_from_console(nutter: &mut pogge::Nutter, input: String) {
    let url = match input.split_whitespace().nth(1) {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    
    let library = match input.split_whitespace().nth(2) {
        Some(data) => data.to_string(),
        None => nutter.library.to_string(),
    };

    match nutter.add_url(url, library) {
        Ok(_) => {},
        Err(error) => println!("Error occured: {}", error),
    };
}

fn add_library_from_console(nutter: &mut pogge::Nutter, input: String) {
    let library = match input.split_whitespace().nth(1) {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };

    match nutter.add_library(library) {
        Ok(_) => {},
        Err(error) => println!("Error occured: {}", error),
    };
}

fn remove_library_from_console(nutter: &mut pogge::Nutter, input: String) {
    let library = match input.split_whitespace().nth(1) {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };

    match nutter.remove_library(library) {
        Ok(_) => {},
        Err(error) => println!("Error occured: {}", error),
    };
}

fn set_library_from_console(nutter: &mut pogge::Nutter, input: String) {
    let library = match input.split_whitespace().nth(1) {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };

    match nutter.set_library(library) {
        Ok(_) => {},
        Err(error) => println!("Error occured: {}", error),
    };
}

fn use_generator_in_console(nutter: &mut pogge::Nutter) {
    let console_input = std::io::stdin();

    if ! nutter.is_library_loaded {
        nutter.load_library_resources(false)
            .build_entities_graph();
    }

    let mut input = String::new();

    while input != "end me" {
        input = "".to_string();

        println!("\n\nGive me a word or type in \"end me\" to finish:");

        console_input.read_line(&mut input).unwrap();
        input.pop();

        let sentence = match nutter.get_sentence_starting_from(input.clone()) {
            Ok(result) => result,
            Err(error) => error,
        };

        println!("{}", sentence);
    };
}

fn print_console_options(library: String, loaded_status: String) {
    println!();
    println!("---CONSOLE OPTIONS \"{}\"/{})---", library, loaded_status);
    println!(":interactive - use interactive nutter console to generate sentences.");
    println!(":set_library {} - set given library name as the current/working library", "{library name}");
    println!(":add_library {} - add new library of URLs to program data.", "{library name}");
    println!(":remove_library {} - remove whole library from program data.", "{library name}");
    println!(":print_libraries - shows a list of all available libraries and URLs they contain.");
    println!(":print_graph - attempts to print all words with connections count on your screen. It will probably overflood it.");
    println!(":add_url {} {} - adds new URL to given library. If library is not provided, the current one will be used.", "{url}", "{library?}");
    println!(":remove_url {} {} - removes given URL from library. If library is not provided, the current one will be used.", "{url}", "{library?}");
    println!(":fetch_data - fetches and loads URLs data for currently selected library. Add -c to clear cached files and re-download everything.");
    println!(":build_graph - builds graph of currently selected library.");
    println!(":print_sentence {} - prints the given number of sentences on the screen without entering interactive mode.", "{starting_word?}");
    println!(":exit - exit program.");
    println!();
}

async fn set_library_from_web(request_body: String) -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Library selected");
}

async fn add_library_from_web(request_body: String) -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Library added");
}

async fn remove_library_from_web(request_body: String) -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Library added");
}

async fn get_library_list_from_web() -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Here, take that list");
}

async fn build_graph_from_web() -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Graph build successful.");
}

async fn add_url_from_web(request_body: String) -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Library added");
}

async fn remove_url_from_web(request_body: String) -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Library added");
}

async fn get_sentence_from_web() -> impl Responder {
    // TODO
    return HttpResponse::Ok().body("Here you go!");
}

#[actix_web::main]
async fn run_as_web_server() -> std::io::Result<()> {
    let state_nutter = web::Data::new(AppStateWithNutter {
        nutter: Mutex::new(pogge::Nutter::init("default".to_string())),
    });

    // TODO add middleware for libraries and urls scope and to building graph too so it's protected
    // with a token
    return HttpServer::new(move || {
        App::new()
            .app_data(state_nutter.clone())
            .service(
                web::scope("/libraries")
                    .route("/", web::get().to(get_library_list_from_web))
                    .route("/set", web::post().to(set_library_from_web))
                    .route("/add", web::post().to(add_library_from_web))
                    .route("/remove", web::post().to(remove_library_from_web))
            )
            .service(
                web::scope("/urls")
                    .route("/add", web::post().to(add_url_from_web))
                    .route("/remove", web::post().to(remove_url_from_web))
            )
            .route("/build_graph", web::post().to(build_graph_from_web))
            .route("/sentence", web::get().to(get_sentence_from_web))
    })
    .bind(("127.0.0.1", 8008))?
    .run()
    .await;
}
