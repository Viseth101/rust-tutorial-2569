fn main() {
    let score:i32 = 39;

    if score >= 60 {
        if score >= 80 {
            println!("Grade A");
        } 
        else {
            println!("Grade B");
        }
    } 
    else {
        if score >= 40 {
            println!("Grade C");
        } 
        else {
            println!("Grade F");
        }
    }
}