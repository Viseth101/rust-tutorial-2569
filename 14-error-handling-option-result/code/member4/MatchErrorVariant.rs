enum AppError {
    UserNotFound,
    DatabaseDown,
}

fn find_user(id: u32) -> Result<String, AppError> {
    match id {
        1 => Ok("Alice".to_string()),
        99 => Err(AppError::DatabaseDown),
        _ => Err(AppError::UserNotFound),
    }
}

fn main() {
    let id = 2; // try 1 (found), 2 (not found), 99 (database down)

    match find_user(id) {
        Ok(name) => println!("Hello {}", name),
        Err(AppError::UserNotFound) => println!("Creating a new user..."),
        Err(AppError::DatabaseDown) => println!("Try again later"),
    }
}