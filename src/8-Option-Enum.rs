fn main() {
    let index = find_first_a(String::from("Hello"));
    match index {
        Some(value) => println!("Index found: {}",value),
        None => println!("a not found in the string"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index,char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}