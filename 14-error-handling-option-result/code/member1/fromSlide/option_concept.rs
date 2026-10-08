fn get_user(id: u32) -> Option<String> {
    if id == 1 {
        return Some("John".to_string());
    } else {
        return None;
    }
}
fn main() {
    match get_user(1) {
        Some(value) => println!("{value}"),
        None => println!("{}", "Not found!"),
    }
}
