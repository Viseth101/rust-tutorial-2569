fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("B can’t be zero".to_string());
    } else {
        return Ok(a / b);
    }
}

fn calculator() -> Result<i32, String> {
    let value = divide(10, 2)?;
    println!("Result is Ok, and continue this path");
    println!("value: {}", value);
    return Ok(value + 100);
}

fn main() {
    match calculator() {
        Ok(value) => println!("{value}"),
        Err(error) => println!("{error}"),
    }
}
