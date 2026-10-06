// Mistake 3 — [Cannot assign to data in an immutable reference]

// Incorrect Code

fn main() {
    let numbers = [10, 20, 30];

    let x = &numbers[0..2];

    x[0] = 100;
}

// Correct Code

fn main() {
    let numbers = [10, 20, 30];

    let x = &mut numbers[0..2];

    x[0] = 100;
}
