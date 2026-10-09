fn main(){
    let score = 99;
    match score {
        100 => println!("Grade A+"),
        97|98|99 => println!("Grade A"),
        80..=96 => println!("Grade A-"),
        60..=79 => println!("Grade B"),
        40..=59 => println!("Grade C"),
        0..=39 => println!("Grade F"),
        _ => println!("Invalid SCORE !"),  // default
    };
}

