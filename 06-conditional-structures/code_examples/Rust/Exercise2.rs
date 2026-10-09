fn main() {
    let score: Option<i32> = Some(68); 

    if let Some(s) = score {
        if s >= 50 {
            println!("Passed with score: {}", s);
        } else {
            println!("Failed with score: {}", s);
        }
    } else {
        println!("No score provided");
    }
}
