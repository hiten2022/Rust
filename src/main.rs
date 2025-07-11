// fn main() {
//     println!("Hello, world!");
// }

use std::thread;
use std::time::Duration;

fn main(){
    thread::spawn(||{
        for i in 1..10 {
            println!("Hello from the spawn thread number: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hello from main thread number: {}",i);
        thread::sleep(Duration::from_millis(1));
    }
}