trait Animal {
    fn name(&self) -> String;              

    fn greet(&self) -> String {             
       format!("Hi, I'm {}", self.name())
    }
}

struct Dog { name: String }

impl Animal for Dog {
    fn name(&self) -> String { self.name.clone() }
}

fn main() {
    let d = Dog { name: String::from("Rex") };
    println!("{}", d.greet());
}
