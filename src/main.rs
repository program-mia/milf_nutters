mod pogge;

fn main() -> Result<(), ()> {
    pogge::run();
    //println!("Status {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);

    return Ok(());
}
