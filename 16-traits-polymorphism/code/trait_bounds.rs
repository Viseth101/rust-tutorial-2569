trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String { String::from("Woof!") }
}

// T: Speak คือ trait bound
fn make_it_speak<T: Speak>(item: T) {
    println!("{}", item.speak());
}

fn main() {
    make_it_speak(Dog);
}
