//This is an example of Consuming Adapter
fn main(){
    let v1 = vec![1,2,3];
    let iter = v1.iter();
    let sum: i32 = iter.sum();
    println!("{}",sum);
}