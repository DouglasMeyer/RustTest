use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("What's your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
        let name: String = name.trim().parse().expect("Please type a name!");
    println!("Hi {:=^20}", name)
}
