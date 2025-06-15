struct User {
    first_name: String,
    last_name: String,
    age: u32
}

fn main() {
    // println!("Hello, world!");
    let user = User{
        first_name: String::from("Hiten"),
        last_name: String::from("Israni"),
        age: 22 
    };

    println!("{}", user.first_name);
}