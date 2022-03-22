fn main() {
    let _prog = std::env::args().next().unwrap();
    let one = std::env::args().skip(1).next().unwrap();
    match one.as_str() {
        "--version" => println!("rail 0.1.0"),
        _ => unimplemented!("What is: {}", one),
    }
}
