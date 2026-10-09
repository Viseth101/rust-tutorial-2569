fn read_port(input: &str) -> u16 {
    input.trim().parse().unwrap() // panics if input is not a valid number
}

fn main() {
    let config = "abc";

    let port = read_port(config);
    println!("Starting server on port {}", port);
}