use std::io;

fn main() {
    println!("What is your name?");

    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let input = input.trim();
        println!("Hello, {input}!");
    }
}
