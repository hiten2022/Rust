fn main(){
    let s1 = String::from("Hello from Hiten!");
    // do_something(s1);

    // This line gives compilation error because variable s1 has been moved to s2 on calling do_something()
    // println!("{}",s1);

    // We can do this
    s1 = do_something(s1);
    println!("{}",s1);
}
fn do_something(s2: String) -> String{
    println!("{}",s2);
    return s2;
}