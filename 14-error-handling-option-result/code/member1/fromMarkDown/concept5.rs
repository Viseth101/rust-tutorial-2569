fn  calculator() ->  Result<i32,String> {
    let value = divide(10,2)? ; //ฟังก์ชันจาก 4.2

    println!("Result is Ok, and continue this path");
    println!("value: {}", value);

    return Ok(value + 100)
}

fn main() {
    match calculator() {
        Ok(value) => println!("{value}"),
        Err(error) => println!("{error}"),
    }
}



// function from concept 2
fn divide(a: i32,b: i32) -> Result<i32,String> {
    if b == 0 {
        return Err("Fail: denominator cannot be zero!".to_string())
    } else {
        return Ok(a / b);
    }
}