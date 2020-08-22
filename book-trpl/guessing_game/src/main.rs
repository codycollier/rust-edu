// a trait
use rand::Rng;

// type
use std::cmp::Ordering;

use std::io;


fn main() {
    println!("");
    println!("Guess the number!");
    println!("");

    // Generate the secret number
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Ssshh! The secret number is: {}", secret);

    loop {

        // Accept a guess and process
        println!("Please enter your guess: ");

        let mut guess_raw = String::new();

        let guess_bytes = io::stdin()
            .read_line(&mut guess_raw)
            .expect("Failed to read line.");

        let guess: i32 = guess_raw
            .trim()
            .parse()
            .expect("Please enter numbers only.");

        println!("You guessed: {} (bytes: {})", guess, guess_bytes);
        println!("");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => println!("You guessed correctly!"),
        }
        println!("");
    }
}
