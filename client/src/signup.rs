use std::io;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn signup() -> () {
    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read input.");

    println!("Please enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read input.");

    // Check the username and password against a database
    let path = Path::new("user_db.csv");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    if let Err(why) = write!(file, "{},{}" , username, password) {
        panic!("Couldn't write to {}: {}", display, why.description());
    }

    println!("Signup successful.");
}
