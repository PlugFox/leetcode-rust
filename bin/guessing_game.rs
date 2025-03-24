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

        let guess: u32 = guess.trim().parse::<u32>().expect("Please enter a number");

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
