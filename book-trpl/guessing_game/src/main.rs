use std::io;

fn main() {
    println!("");
    println!("Guess the number!");
    println!("");
    println!("Please enter your guess: ");

    let mut guess = String::new();

    let guess_bytes = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
    println!("(bytes: {})", guess_bytes);
}
