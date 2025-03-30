use std::cmp::Ordering;
use std::io::{self, Write};

fn ask_for_number() -> io::Result<u32> {
    print!("Guess a number between 0 and 100: ");
    io::stdout().flush()?;

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    let guess: u32 = guess
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    return Ok(guess);
}

fn choose_number(min: u32, max: u32) -> u32 {
    rand::random_range(min..max)
}

fn main() {
    println!("Welcome to guess the number game!\n");

    let mut number = choose_number(0, 100);

    loop {
        loop {
            let guess = match ask_for_number() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            match guess.cmp(&number) {
                Ordering::Less => println!("Too low!"),
                Ordering::Greater => println!("Too high!"),
                Ordering::Equal => {
                    println!("Congrats! You guessed the number {number} correctly.");
                    break;
                }
            }
        }

        println!("Want to play again? (y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let input = input.trim();
        match input {
            "y" | "Y" => {
                number = choose_number(0, 100);
            }
            _ => break,
        }
    }

    println!("\nGoodbye!");
}
