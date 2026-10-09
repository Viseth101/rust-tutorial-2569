trait Animal {
    fn speak(&self);
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
fn make_sound<T: Animal>(animal: T) {
    animal.speak();
}
fn main() {
    let dog = Dog;
    let cat = Cat;

    make_sound(dog);
    make_sound(cat);
}
