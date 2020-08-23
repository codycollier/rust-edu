use rand::thread_rng;
use rand::Rng;

fn main() {
    println!("");

    let mut rng = thread_rng();
    let number = rng.gen_range(0, 11);
    println!("random number: {}", number);

    // loop forever (until ctrl-c interrupt)
    //loop {
    //    println!("rust is cool!");
    //}

    // loop with break and return val
    let mut counter = 0;
    let result = loop {
        if counter == number {
            break counter;
        }
        counter += 1;
    };
    println!("      counter: {}", result);
    println!("");

    // while loop
    let mut countdown = 3;
    while countdown != 0 {
        println!("{}", countdown);
        countdown -= 1;
    }
    println!("Lift off!");
    println!("");

    // indexing through an array
    let arr = [1, 3, 5, 7, 9];
    let mut index = 0;
    while index < arr.len() {
        println!("arr[{}]: {}", index, arr[index]);
        index += 1;
    }
}
