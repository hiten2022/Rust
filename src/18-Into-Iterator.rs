fn main(){
    // Takes ownership of vector, instead of just borrowing. This has some performance benefits.
    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.into_iter();
    for val in v1_iter{
        println!("Got: {}",val);
    }
}