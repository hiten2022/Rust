fn main(){
    let s1 = String::from("Hello from Hiten!");
    do_something(&s1);
    println!("Calling from main: {}",s1);
}

fn do_something(s2: &String){
    println!("Calling from do_something: {}",s2);
}