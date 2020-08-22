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
    println!("MAX_POINTS: {}", MAX_POINTS);

    // numeric literals
    println!("number: {}", 255);
    println!("number: {}", 0xff);
    println!("number: {}", 0o377);
    println!("number: {}", 0b1111_1111);
    println!("number: {}", b'Z');
    // println!("number: {}", b'■');
    // println!("number: {}", b'ß');

    println!("number: {}", 255u32);
    println!("number: {}", 255i32);
    println!("number: {}", 255usize);
    println!("number: {}", 255isize);
}
