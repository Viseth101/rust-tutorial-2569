

//option
fn find_first_a(text: &str) -> Option<usize>{
    text.find('a') 
}

//result
#[derive(Debug)]
struct DivisionError{
    message: String,
}
fn divide(numerator: f64, denominator: f64) -> Result<f64, DivisionError>{
    if denominator == 0.0 {
        Err(DivisionError{
            message: "Cannot divide by Zero ".to_string(),
        })
    }else {
        Ok(numerator / denominator)
    }
}

// ? operator + Error Propagation
fn calculate_division_then_add_one(num: f64, den: f64)->Result<f64,DivisionError>{
    let result = divide(num, den)?;
    Ok(result +1.0)
}

fn main() {
    

    //option
    match find_first_a("Hello World!"){
        Some(index) => println!("The first 'a' is at index {}", index),
        None => println!("No 'a' found in the text."),
    }

    //result
    println!("{:?}",divide(5.0,2.0));
    println!("{:?}",divide(5.0,0.0));

    //? operator + Error Proagation
    println!("{:?}", calculate_division_then_add_one(5.0,2.0));
    println!("{:?}", calculate_division_then_add_one(5.0,0.0));

}
