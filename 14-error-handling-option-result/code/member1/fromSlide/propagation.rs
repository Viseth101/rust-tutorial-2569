fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("B can’t be zero".to_string());
    } else {
        return Ok(a / b);
    }
}

fn calculate() -> Result<i32, String> {
    match divide(10, 0) {
        Ok(value) => {
            println!("success!!");
            return Ok(value);
        }
        Err(error) => return Err(error),
    }
}

fn main() {
    match calculate() {
        Ok(value) => println!("Result:{value}"),
        Err(error) => println!("Error:{error}"),
    }
}
