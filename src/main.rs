mod pogge;

fn main() -> Result<(), ()> {
    let nutter = pogge::Nutter::init("default".to_string());
    nutter.load_library();
    // TODO accept some initial parameters and pass them to the pogge init method to start
    // everything, etc? or at least initial data
    pogge::run();
    //println!("Status {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);

    return Ok(());
}
