fn main() {
    let score:i32 = 90;
    
    if score >= 80{
        println!("Grade A");
    }
    else if score >= 60{
        println!("Grade B");
    }
    else if score >= 40{
        println!("Grade C");
    }
    else{
        println!("Grade F");
    }
}