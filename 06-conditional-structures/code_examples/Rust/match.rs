fn main() {
    let grade = "X";

    match grade {
        "A" => println!("Excellent"),
        "B" => println!("Good"),
        "C" => println!("Okay"),
        _ => println!("Unknown grade"),
    }
}