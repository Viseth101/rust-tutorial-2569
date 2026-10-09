use std::num::ParseIntError;

fn read_port(input: &str) -> Result<u16, ParseIntError> {
    let port: u16 = input.trim().parse()?; // returns Err to the caller instead of panicking
    Ok(port)
}

fn main() {
    let config = "abc";

    match read_port(config) {
        Ok(port) => println!("Starting server on port {}", port),
        Err(e) => println!("Invalid config: {}", e),
    }
}