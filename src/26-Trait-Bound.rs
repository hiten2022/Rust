pub trait Summary {
    fn summarize(&self) -> String{
        return format!("Default implementation for the summarize function");
    }
}

pub trait Fix {
    fn fixing(&self) -> String{
        return format!("I have fixed this in the default way");
    }
}

struct User{
    name: String,
    age: u32
}

struct Student {
    name: String,
    rollNo: u32
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("Name of the user is {} and their age is {}", self.name, self.age);
    }
}
impl Fix for User{}


// pub fn notify(item: &impl Summary) -> String{
//     return format!("This is a notification from {}", item.summarize());
// }

// Another implementation of the above notify function is using trait bound like this:
pub fn notify<T: Summary>(item: &T) -> String{
    // Here the T generic is bound to the trait summary
    return format!("This is a notification from {}",item.summarize());
}

// It can also be bound to multiple traits like this: <T: Summary + Fix>
pub fn notifyV2<T: Summary + Fix>(item: &T) -> String{
    return format!("{} {}", item.summarize(), item.fixing());
}

fn main() {
    let user = User {
        name: String::from("Hiten"),
        age: 22
    };
    println!("{}",notifyV2(&user));
}