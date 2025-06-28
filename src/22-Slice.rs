fn main() {
    let mut word = String::from("Hello World!");
    let word2 = &word[0..5];

    // We can have multiple immutable references at once
    // If we already have a mutable reference, we can't have another mutable/immutable reference
    // word.clear();

    println!("{}",word2);
}