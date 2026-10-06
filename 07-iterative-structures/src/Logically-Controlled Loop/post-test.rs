use std::io;
fn main(){

    loop{
       let mut msg = String::new();
        io::stdin()
           .read_line(&mut msg)
            .expect("Failed input");

        let numbers: i32 = match msg.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        if numbers < 0{
            println!("Stopped! Entered negative numbers: {}",  numbers);
            break;
        }    

        println!("Input numbers: {}", numbers);
    }
}