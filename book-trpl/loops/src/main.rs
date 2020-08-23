use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    let number = rng.gen_range(0, 11);
    println!("random number: {}", number);

    //loop {
    //    println!("rust is cool!");
    //}

    let mut counter = 0;

    let result = loop {
        if counter == number {
            break counter;
        }
        counter += 1;
    };
    println!("      counter: {}", result);
}
