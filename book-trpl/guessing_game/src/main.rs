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

    let mut guess = String::new();

    let guess_bytes = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
    println!("(bytes: {})", guess_bytes);
    println!("");

    if guess == secret {
        println!("You got it!");
    } else {
        println!("Incorrect, better luck next time!");
    }
}
