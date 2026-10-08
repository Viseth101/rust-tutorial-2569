trait Speak {         
    fn speak(&self) -> String;
}

struct Dog;             

impl Speak for Dog {    
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}
fn main() {
    let d = Dog ;
    println!("{}", d.speak());
}
