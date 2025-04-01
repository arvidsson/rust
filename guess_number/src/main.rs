use core::num;
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write};

fn ask_for_number() -> Result<u32, Box<dyn Error>> {
    print!("Guess a number between 0 and 100: ");
    io::stdout().flush()?;

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    let guess: u32 = guess.trim().parse()?;
    return Ok(guess);
}

fn choose_number() -> u32 {
    rand::random_range(0..=100)
}

fn ask_to_play_again() -> Result<bool, Box<dyn Error>> {
    println!("Want to play again? (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    return match input {
        "y" | "Y" => Ok(true),
        _ => Ok(false),
    };
}

fn game_loop() -> Result<(), Box<dyn Error>> {
    let mut number = choose_number();
    let mut num_guess = 0;

    loop {
        loop {
            let guess = match ask_for_number() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            num_guess += 1;

            match guess.cmp(&number) {
                Ordering::Less => println!("Too low!"),
                Ordering::Greater => println!("Too high!"),
                Ordering::Equal => {
                    println!(
                        "\nCongrats! You guessed the number {number} in {num_guess} attempts."
                    );
                    break;
                }
            }
        }

        if ask_to_play_again()? {
            number = choose_number();
            num_guess = 0;
        } else {
            break;
        }
    }

    Ok(())
}

fn main() {
    println!("Welcome to guess the number game!\n");

    if let Err(e) = game_loop() {
        println!("Error: {e}");
        return;
    }

    println!("\nGoodbye!");
}
