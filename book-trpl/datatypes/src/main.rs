const MAX_POINTS: u32 = 100_000;

fn main() {
    // constant
    println!("MAX_POINTS: {}", MAX_POINTS);

    // number / integer literals
    println!("number: {}", 255);
    println!("number: {}", 0xff);
    println!("number: {}", 0o377);
    println!("number: {}", 0b1111_1111);
    println!("number: {}", b'Z');
    // println!("number: {}", b'■');
    // println!("number: {}", b'ß');

    // sample of some different forms
    println!("number: {}", 255u32);
    println!("number: {}", 255i32);
    println!("number: {}", 255usize);
    println!("number: {}", 255isize);

    // floats (32 single precision, 64 double precision)
    println!("float  : {}", 0.01234567890123456789);
    println!("float32: {}", 0.01234567890123456789f32);
    println!("float64: {}", 0.01234567890123456789f64);

    // operations
    let sum = 500 + 100;
    let difference = 512.1 - 256.0;
    let product = 3 * 8;
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);

    // rounds off (similar to python)
    let quotient = 56 / 57;
    println!("quotient: {}", quotient);

    // for precision
    let quotient = 56.0 / 57.0;
    println!("quotient: {}", quotient);

    // with floats
    let quotient = 56.0 / 57.1;
    println!("quotient: {}", quotient);

    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}
