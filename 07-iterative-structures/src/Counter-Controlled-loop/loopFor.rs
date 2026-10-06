

fn main(){
    let mut count = 0;
    let mut numbers = Vec::new();
    for i in 0..=10 {
        count += i;
        numbers.push(count);
        println!("numbers = {}", numbers[i]);
    }
}


