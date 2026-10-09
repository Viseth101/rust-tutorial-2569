trait Named {
    fn name(&self) -> String;
}

trait Greet: Named {
    fn greet(&self) -> String {
        format!("Hi, {}", self.name())
    }
}

struct Person { name: String }

impl Named for Person {
    fn name(&self) -> String { self.name.clone() }
}

impl Greet for Person {}

fn main() {
    let p = Person { name: String::from("Alice") };
    println!("{}", p.greet());
}
