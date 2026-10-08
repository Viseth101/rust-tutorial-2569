use std::num::ParseIntError;

fn parse_and_double(text: &str) -> Result<i32, ParseIntError> {
    let number = text.parse::<i32>()?; 
    Ok(number * 2)
}

fn main() {
    let input = "abc"; 

    match parse_and_double(input) {
        Ok(val) => println!("Success: {}", val),
        Err(err) => println!("Parsing Failure: {}", err),
    }
}
