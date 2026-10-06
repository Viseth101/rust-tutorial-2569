use std::mem::size_of;
use std::cell::RefCell;
fn main() {
    println!("=== Size Comparison of Stack ===");

    let raw_data: i32 = 42;
    println!("Size of i32 :  {} bytes", size_of::<i32>());

    let reference: &i32 = &raw_data;
    println!("Size of &i32 (Normal Pointer): {} bytes", size_of::<&i32>());

    let ref_cell: RefCell<i32> = RefCell::new(42);
    println!("Size of RefCell<i32> (Smart Pointer): {} bytes", size_of::<RefCell<i32>>());
}
