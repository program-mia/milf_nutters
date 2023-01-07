mod pogge;

fn main() -> Result<(), ()> {
    let mut nutter = pogge::Nutter::init("default".to_string());

    nutter.load_library_resources()
        .build_entities_graph();
//        .print_graph();

    let mut input = String::new();
    let stdin = std::io::stdin();

    while input != "end me" {
        input = "".to_string();

        println!("\n\nGive me a word or type in \"end me\" to finish:");

        stdin.read_line(&mut input).unwrap();
        input.pop();

        nutter.print_sentence_starting_from(input.clone());
    }
    // TODO accept some initial parameters and pass them to the pogge init method to start
    // everything, etc? or at least initial data
    return Ok(());
}
