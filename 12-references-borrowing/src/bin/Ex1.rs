fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    names.push(String::from("Quinn"));
    println!("Hosted by: {}", first);

    for name in &names {
        if name == "Oliver" {
            names.push(String::from("Charlotte"));
        }
    }
    println!("{:?}", names);
}