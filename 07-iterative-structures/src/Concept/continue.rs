fn main() {
    for number in 1..=5 {
        if number % 2 == 0 {
            continue;
        }

        println!("{number}");
    }
}