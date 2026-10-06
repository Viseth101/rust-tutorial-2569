//Array

```rust
fn main() {
    let scores: [i32; 3] = [80, 90, 75];

    println!("{}", scores[0]);
}
```
// เปลี่ยนค่าของข้อมูลได้ถ้าประกาศตัวแปรด้วย `mut`

```rust
fn main() {
    let mut scores = [80, 90, 75];

    scores[0] = 85;

    println!("{}", scores[0]);
}
```
//Tuple

```rust
fn main() {
    let student = ("Alice", 20, true);

    println!("{}", student.0);
}
```
//Vector

```rust
fn main() {
    let mut scores = vec![80, 90, 75];

    scores.push(85);

    println!("{:?}", scores);
}
```
//String

```rust
fn main() {
    let mut text = String::from("Hello");

    text.push_str(" Rust");

    println!("{}", text);
}
```
//&str

```rust
fn main() {
    let name: &str = "Alice";

    println!("{}", name);
}
```
