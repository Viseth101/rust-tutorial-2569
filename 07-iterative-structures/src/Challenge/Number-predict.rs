use std::cmp::Ordering;

fn main() {
    let secret = 37;
    let guesses = [50, 25, 40, 37];
    let max_tries = 5;
    let limit = max_tries.min(guesses.len()); 

    let mut tries = 0;
    let mut won = false;

    loop {
        let guess = guesses[tries];
        tries += 1;

        match guess.cmp(&secret) {
            Ordering::Greater => println!("ครั้งที่ {}: ทาย {} -> มากไป", tries, guess),
            Ordering::Less => println!("ครั้งที่ {}: ทาย {} -> น้อยไป", tries, guess),
            Ordering::Equal => {
                println!("ครั้งที่ {}: ทาย {} -> ถูกต้อง!", tries, guess);
                won = true;
            }
        }

        if won || tries >= limit {
            break;
        }
    }

    if won {
        println!("ชนะ! ใช้ {} ครั้ง", tries);
    } else {
        println!("แพ้! ใช้ครบ {} ครั้งแล้ว", tries);
    }
}