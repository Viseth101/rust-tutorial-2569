// Mistake 2 — [Range Out of Bounds]

// Incorrect Code

fn main() {
    let numbers = [10, 20, 30];

    let x = &numbers[1..4]; 
}

//Correct Code

fn main() {
    let numbers = [10, 20, 30];

    let x = &numbers[0..3]; 
}
