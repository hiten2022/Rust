use std::fs::read_to_string; 

fn main() {
    let result = read_to_string("a.txt");

    match result {
        Ok(data) => println!("The content of the file is: {}", data),
        Err(err) => println!("Couldn't read the file with the error: {}", err),
    }
}