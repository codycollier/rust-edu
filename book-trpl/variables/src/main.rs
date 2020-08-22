const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    println!("x is: {}", x);

    // not allowed. not mutable.
    // x = 6;
    // println!("x is: {}", x);

    // shadow (inc type change)
    let x = "six";
    println!("x is: {}", x);

    // shadow + mutable
    let mut x = 6;
    println!("x is: {}", x);

    x = 7;
    println!("x is: {}", x);

    // constant
    println!("MAX_POINTS: {}", MAX_POINTS)
}
