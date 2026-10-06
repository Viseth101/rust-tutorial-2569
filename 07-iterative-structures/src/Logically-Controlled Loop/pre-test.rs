use std::io;
fn main(){
     let mut number: i32 = 0;

    while number >= 0 {
        let mut msg = String::new();
        println!("Enter a number (negative number to stop):");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed input");

        number = match msg.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                0
            }
        };
        if number > 0{
            println!("Input = {}", number);
        }
    }

    println!("Stopped! You entered negative number: {}", number);

}