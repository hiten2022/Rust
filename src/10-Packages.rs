use chrono::{Local, Utc};

fn main(){
    let now = Utc::now();
    println!("current date and time is : {}",now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time is: {}",formatted);

    let local_time = Local::now();
    println!("Local time is: {}",local_time);
}