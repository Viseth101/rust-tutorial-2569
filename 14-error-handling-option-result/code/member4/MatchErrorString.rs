fn find_user(id: u32) -> Result<String, String> {
    if id == 1 {
        Ok("Alice".to_string())
    } else {
        Err("user not found".to_string())
    }
}

fn main() {
    match find_user(2) {
        Ok(name) => println!("Hello {}", name),
        Err(err) => {
            if err.contains("not found") {
                println!("Creating a new user...");
            } else {
                println!("Something else went wrong");
            }
        }
    }
}