use rand::seq::SliceRandom;
use rand::Rng;

fn main() {
    // random gen from a range
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0, 10);
    println!("number: {:?}", number);

    if number < 5 {
        println!("condition is true (< 5)");
    } else {
        println!("condition is false (>= 5)");
    }

    // random choice
    // let number = numbers.choose(&mut rng).unwrap();
    // if *number < 5 {
    let numbers = [1, 3, 5, 7];
    let mut rng = rand::thread_rng();
    let &number = numbers.choose(&mut rng).unwrap();
    println!("number: {:?}", number);

    if number < 5 {
        println!("condition is true (< 5)");
    } else if number == 5 {
        println!("condition is true (= 5)");
    } else {
        println!("conditions are false (> 5)");
    }

    // if condition as an expression
    let num = if true { 5 } else { 7 };
    println!("num is: {}", num);
}
