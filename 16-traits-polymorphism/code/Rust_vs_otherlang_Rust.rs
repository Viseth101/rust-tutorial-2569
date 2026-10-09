struct Person {
    name : String
}
struct Cat {
    name : String
}

trait Eat{
    fn eat_dinner(&self);
}

impl Eat for Person{
    fn eat_dinner(&self){
        println!("{} Yummy",self.name);
    }
}

impl Eat for Cat{
    fn eat_dinner(&self){
        println!("{} Num Num NUm",self.name);
    }
}

fn main() {
   let perr = Person{
    name : String::from("Coco")
   };
   perr.eat_dinner();
   let catt = Cat{
    name : String::from("Mr.Joe")
   };
    catt.eat_dinner();
}
