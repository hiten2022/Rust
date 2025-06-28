struct User<'a> {
    name: &'a str
}

fn main() {
    let name = String::from("Hiten");
    let user = User {
        name: &name
    };
    println!("{}",user.name);
}