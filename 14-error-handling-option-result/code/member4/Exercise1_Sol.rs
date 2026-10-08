fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    match first_even(&[1, 3, 4, 7]) {
        Some(n) => println!("First even number: {}", n),
        None => println!("No even number found."),
    }

    match first_even(&[1, 3, 5]) {
        Some(n) => println!("First even number: {}", n),
        None => println!("No even number found."),
    }
}