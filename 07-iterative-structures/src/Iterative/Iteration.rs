fn main(){
    let names = vec!["Bob", "Carol","Ted"];

    for name in &names{
        println!("Names --> {}", name);
    }
}