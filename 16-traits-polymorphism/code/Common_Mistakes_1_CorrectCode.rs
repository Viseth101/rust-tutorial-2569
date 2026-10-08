trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn make_sound<T: Animal>(animal: T) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    make_sound(dog);
}
