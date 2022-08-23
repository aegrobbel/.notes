use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number (between 1 and 100)...");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too large."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
