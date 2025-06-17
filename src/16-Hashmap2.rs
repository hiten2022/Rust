use std::collections::HashMap;

fn main(){
    let pairs = vec![
        (String::from("Hiten"),22),
        (String::from("Raman"),32),
    ];
    let map = get_key_value_pairs(pairs);
    println!("{:?}",map);
}

fn get_key_value_pairs(vec: Vec<(String,i32)>) -> HashMap<String,i32>{
    let mut hm = HashMap::new();
    for (key,val) in vec {
        hm.insert(key,val);
    }
    return hm;
}