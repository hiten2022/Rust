// fn longest(a: &str, b: &str) -> &str{
//     if a.len()>b.len(){
//         return a;
//     }
//     else{
//         return b;
//     }
// }

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str{
    if str1.len()>str2.len(){
        return str1;
    }
    else{
        return str2;
    }
}

fn main(){
    let str1 = String::from("Israni");
    let longest_str;
    {
        let str2 = String::from("Hiten");
        longest_str = longest(&str1,&str2);
    }
    println!("The longest string among the two is: {}", longest_str);
}