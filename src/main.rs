mod pogge;

use std::env;

enum RunningMode {
    Console,
    WebServer,
}

fn main() -> Result<(), ()> {
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

fn run_in_console() -> Result<(), ()> {
    let console_input = std::io::stdin();
    let mut option: String = String::new();
    let mut library: String = "default".to_string();

    let mut nutter = pogge::Nutter::init(library);

    while option != ":exit" {
        option = String::new();

        print_console_options(nutter.library.clone());

        console_input.read_line(&mut option).unwrap();
        option.pop();

        let action = match option.clone().split_whitespace().nth(0) {
            Some(data) => data.to_string(),
            None => "".to_string(),
        };

        match action.as_str() {
            ":interactive" => use_generator_in_console(&mut nutter),
            ":set_library" => set_library_from_console(&mut nutter, option.clone()),
            ":print_libraries" => print_libraries(&nutter),
            ":exit" => option = action.to_string().clone(),
            _ => println!("This option does not exists, please try again."),
        };
    };


    return Ok(());
}

fn print_libraries(nutter: &pogge::Nutter) {
    for library in nutter.get_libraries_list() {
        println!("Library - {}", library.name);

        for url in library.urls {
            println!("\t- {}", url);
        }
    }
}

fn set_library_from_console(nutter: &mut pogge::Nutter, input: String) {
    // TODO
}

fn use_generator_in_console(nutter: &mut pogge::Nutter) {
    let console_input = std::io::stdin();

    nutter.load_library_resources()
        .build_entities_graph();
//        .print_graph();

    let mut input = String::new();

    while input != "end me" {
        input = "".to_string();

        println!("\n\nGive me a word or type in \"end me\" to finish:");

        console_input.read_line(&mut input).unwrap();
        input.pop();

        nutter.print_sentence_starting_from(input.clone());
    };
}

fn print_console_options(library: String) {
    // TODO add some flag to Nutter indicating if the graph for the selected library is built and
    // up to date
    println!("---CONSOLE OPTIONS (current/working library - {})---", library);
    println!(":interactive - use interactive nutter console to generate sentences.");
    println!(":set_library {} - set given library name as the current/working library", "{library name}");
    println!(":add_library {} - add new library of URLs to program data.", "{library name}");
    println!(":remove_library {} - remove whole library from program data.", "{library name}");
    println!(":print_libraries - shows a list of all available libraries and URLs they contain.");
    println!(":add_url {} {} - adds new URL to given library. If library is not provided, the current one will be used.", "{url}", "{library?}");
    println!(":remove_url {} {} - removes given URL from library. If library is not provided, the current one will be used.", "{url}", "{library?}");
    println!(":fetch_data - fetches and loads URLs data for currently selected library. Use add -c to clear cached files and re-download everything.");
    println!(":build_graph - builds graph of currently selected library.");
    println!(":print_sentences {} - prints the given number of sentences on the screen without entering interactive mode.", "{number}");
    println!(":exit - exit program.");
}

fn run_as_web_server() -> Result<(), ()> {
    // TODO
    println!("This is not done at all yet, will slap Actix on it later on.");

    return Ok(());
}
