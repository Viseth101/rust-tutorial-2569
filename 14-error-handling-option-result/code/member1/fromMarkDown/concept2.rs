fn divide(a: i32,b: i32) -> Result<i32,String> {
    if b == 0 {
        return Err("Fail: denominator cannot be zero!".to_string())
    } else {
        return Ok(a / b);
    }
}

fn main() {
    let _varaint_return = divide(10,2);
}