
use std::io;
fn main() {

    loop {
        let mut input = String::new();

        println!("Enter a number (negative to stop):");

        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let number: i32 = input.trim().parse().unwrap();

        if number < 0 {
            println!("Program stopped.");
            break;
        }
        println!("You entered: {}", number);
    }
}