fn cause_panic(value: i32){
    if value == 0{
        panic!("Panic! The value was zero, which is not allowed here");
    }
    println!("Value{} is fine.",value);
}
fn main() {
    cause_panic(0); 
}
