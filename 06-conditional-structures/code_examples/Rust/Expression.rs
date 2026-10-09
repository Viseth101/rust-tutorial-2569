fn main() {
    let time = 20;

    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };

    println!("{}", greeting);
}