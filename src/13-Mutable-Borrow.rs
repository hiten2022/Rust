fn main(){
    let mut s1 = String::from("Hello from Hiten!");
    do_something(&mut s1);
    println!("Calling from main: {}",s1);
}

fn do_something(s2: &mut String){
    s2.push_str(" Hi there.");
    println!("Calling from do_something function: {}",s2);
}

// We can either have one mutable reference of any number of immutable references