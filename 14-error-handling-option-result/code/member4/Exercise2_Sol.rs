use std::num::ParseIntError;

fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("10", "20"));
    println!("{:?}", add_strings("10", "abc"));
}