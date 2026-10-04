use std::rc::Rc;

#[derive(Debug)]
struct Owner {
    name: String,
}

fn main() {
    let owner = Rc::new(Owner {
        name: String::from("Shared Resource"),
    });

    println!("Reference count after creation = {}", Rc::strong_count(&owner));

    let owner_clone1 = Rc::clone(&owner);
    println!("Reference count after clone1 = {}", Rc::strong_count(&owner));

    {
        let owner_clone2 = Rc::clone(&owner);
        println!("Reference count after clone2 = {}", Rc::strong_count(&owner));
        println!("owner_clone2 points to: {:?}", owner_clone2);
    }

    println!("Reference count after clone2 dropped = {}", Rc::strong_count(&owner));
    println!("owner = {:?}, owner_clone1 = {:?}", owner, owner_clone1);
}
