#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn sum_list(list: &List) -> i32 {
    match list {
        Cons(value, next) => value + sum_list(next),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("List: {:?}", list);
    println!("Sum of list = {}", sum_list(&list));

    let boxed_number = Box::new(42);
    println!("Boxed number = {}", boxed_number);

}
