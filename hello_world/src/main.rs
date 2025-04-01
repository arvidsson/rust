use std::io;

fn main() {
    println!("What is your name?");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            if input.is_empty() {
                println!("Error: Input is empty!");
                return;
            }
            println!("Hello, {input}!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
