use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Counter {
    value: i32,
}

fn increment(shared_counter: &Rc<RefCell<Counter>>) {
    let mut counter = shared_counter.borrow_mut();
    counter.value += 1;
}

fn main() {
    let shared_counter = Rc::new(RefCell::new(Counter { value: 0 }));

    let counter_a = Rc::clone(&shared_counter);
    let counter_b = Rc::clone(&shared_counter);

    increment(&counter_a);
    increment(&counter_b);
    increment(&shared_counter);

    println!("Final value = {}", shared_counter.borrow().value);
    println!("Total owners (strong_count) = {}", Rc::strong_count(&shared_counter));
}
