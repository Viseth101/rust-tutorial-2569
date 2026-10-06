// Mistake 4 — [cannot borrow numbers as mutable because it is also borrowed as immutable]

// Incorrect Code

fn main() {
    let mut numbers = vec![1, 2, 3];

    for num in &numbers { // ยืมอ่านแบบ Immutable Borrow
        if  num == 2 {
            numbers.push(4); //Error: cannot borrow `numbers` as mutable
        }
    }
}

// Correct Code

fn main() {
    let mut numbers = vec![1, 2, 3];
    let len = numbers.len(); ก่อน

    for i in 0..len {
        if numbers[i] == 2 {
            numbers.push(4);
        }
    }

    println!("{:?}", numbers); 
}
