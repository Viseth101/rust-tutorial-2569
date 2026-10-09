trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn make_sound(animal: &dyn Speak) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    make_sound(&dog);
}
