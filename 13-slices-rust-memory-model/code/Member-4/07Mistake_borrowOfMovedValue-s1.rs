// Mistake 1 — [borrow of moved value: s1]
// Incorrect Code

fn main() {
    let s1 = String::from("hello"); // heap-allocated
    let s2 = s1; // move, s1 ใช้ไม่ได้อีก
    println!("{}", s1); 
}

//Correct Code

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let s2 = &s1;

