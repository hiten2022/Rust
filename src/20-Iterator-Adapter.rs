// This is an Iterator Adapter
fn main(){
    let v1 = vec![1,2,3,4];
    let iter = v1.iter();
    let iter2 = iter.map(|x| x+1);
    // for val in iter2 {
    //     println!("{}",val);
    // }
    let iter3 = iter2.filter(|x| *x%2 == 0);
    for val in iter3 {
        println!("{}",val);
    }
}