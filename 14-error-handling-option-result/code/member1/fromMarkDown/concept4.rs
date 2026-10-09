fn calculate() -> Result<i32, String> {
    match divide(10,0) { //ฟังก์ชันจาก 4.2
        Ok(value) => {
            println!("success!!");
            return Ok(value);
        },
        Err(error) => return Err(error),
        
    }
}

fn main() {
    match calculate() {
        Ok(value) => println!("Result:{value}"),
        Err(error) => println!("Error:{error}"),
    }
}




// function from concept2
fn divide(a: i32,b: i32) -> Result<i32,String> {
    if b == 0 {
        return Err("Fail: denominator cannot be zero!".to_string())
    } else {
        return Ok(a / b);
    }
}