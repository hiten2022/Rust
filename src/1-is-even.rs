fn main() {
    println!("{}",is_even(-19));
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    else{
        return false; 
    } 
}