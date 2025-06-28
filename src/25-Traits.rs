// This is kind of like a interface in Java or an abstract class in other languages.
pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("The user {} is of the age {}",self.name,self.age);
    }
}

fn main(){
    let user = User{
        name: String::from("Hiten"),
        age: 22
    };
    println!("{}",user.summarize());
}