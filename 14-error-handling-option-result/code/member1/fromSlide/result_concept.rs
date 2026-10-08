fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("B can’t be zero".to_string());
    } else {
        return Ok(a / b);
    }
}
fn main() {
    match divide(10, 2) {
        Ok(value) => println!("{value}"),
        Err(error) => println!("{error}"),
    }
}
