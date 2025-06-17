fn main(){
    // Immutable Reference:
    // let v1 = vec![1,2,3,4,5];
    // let v1_iter = v1.iter();
    // for val in v1_iter{
    //     println!("Got: {}",val);
    // }

    //Iter_mut
    // let mut v1 = vec![1,2,3,4,5];
    // let mut v1_iter = v1.iter_mut();
    // for val in v1_iter {
    //     *val = *val + 1;
    // }
    // println!("{:?}",v1);

    // Using Actual Way to Iterate Using Iterator
    let v1 = vec![1,2,3,4,5];
    let mut iter = v1.iter();
    while let Some(val) = iter.next() {
        print!("{}",val);
    }
}