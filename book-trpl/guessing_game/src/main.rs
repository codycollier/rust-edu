use rand::Rng;
use std::io;

fn main() {
    println!("");
    println!("Guess the number!");
    println!("");

    // Generate the secret number
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Ssshh! The secret number is: {}", secret);

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

    if guess == secret {
        println!("You got it!");
    } else {
        println!("Incorrect, better luck next time!");
    }
}
