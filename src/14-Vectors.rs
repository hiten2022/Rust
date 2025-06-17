fn main(){
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    let mut vec = vec![1,2,3];
    println!("{:?}",even_filter(&vec));
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0{
            new_vec.push(*val);
        }
    }
    return new_vec;
}