fn main() {
    let variant: Result<String, String> = Result::Ok("hello world!".to_string());
    match variant {
        Ok(value) => println!("Result: {value}"),
        Err(_error) => println!("Error: something wrong"),
    }
}
