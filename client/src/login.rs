use std::io;

pub fn login() -> bool {
    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read input.");

    println!("Please enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read input.");

    // Check the username and password against a database or hardcoded values
    let valid_username = "myusername\n";
    let valid_password = "mypassword\n";

    if username == valid_username && password == valid_password {
        println!("Login successful.");
        return true;
    } else {
        println!("Invalid username or password.");
        return false;
    }
}
