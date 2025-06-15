fn get_string_length_in_chars(s: &str) -> usize {
    s.chars().count()
}

fn main(){
    let my_string = String::from("Hello, world!");
    let length = get_string_length_in_chars(&my_string);
    println!("{}",length);
}