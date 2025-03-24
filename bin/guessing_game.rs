use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("> Guess the number!");
    println!("> Please input your guess.");

    let secret_number = rand::rng().random_range::<u32, _>(1..=100);

    println!("> The secret number is: {}", secret_number);

    let mut guess = String::new();

    println!("> Please enter your guess:");

    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Check for exit commands
        match guess.trim() {
            "exit\n" | "q" | "quit" => {
                println!("> Exiting the game.");
                return;
            }
            _ => {}
        }

        // Parse the guess
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("> Please enter a valid number.");
                continue;
            }
        };

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("> Too small!"),
            Ordering::Greater => println!("> Too big!"),
            Ordering::Equal => {
                println!("> You guessed it!");
                break;
            }
        }
    }

    //println!("You guessed: {}", guess);
}
