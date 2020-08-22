const MAX_POINTS: u32 = 100_000;

fn main() {
    // constant
    println!("MAX_POINTS: {}", MAX_POINTS);

    //-----------------------------------------------------------------------
    // scalar types
    //-----------------------------------------------------------------------

    // number / integer literals
    println!("integer: {}", 255);
    println!("integer: {}", 0xff);
    println!("integer: {}", 0o377);
    println!("integer: {}", 0b1111_1111);
    println!("integer: {}", b'Z');
    // println!("integer: {}", b'■');
    // println!("integer: {}", b'ß');

    // sample of some different forms
    println!("integer: {}", 255u32);
    println!("integer: {}", 255i32);
    println!("integer: {}", 255usize);
    println!("integer: {}", 255isize);

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

    // booleans
    let mut foo = true;
    println!("bool: {}", foo);

    foo = false;
    println!("bool: {}", foo);

    let foo: bool = true;
    println!("bool: {}", foo);

    // char
    // char literals in single quotes ''.  strings in "".
    let bar = 'Z';
    println!("char: {}", bar);

    let bar = 'ℤ';
    println!("char: {}", bar);

    // char is 4 byte unicode scalar value
    let heart_eyed_cat = '😻';
    println!("char: {}", heart_eyed_cat);

    //-----------------------------------------------------------------------
    // primitive compound types
    // tuples and arrays
    // both are fixed length
    // tuples allow mixed types
    // arrays must contain elements all of same type
    //-----------------------------------------------------------------------

    // tuple
    let tup: (i32, f32, &str) = (1, 2.3, "four");
    println!("tup: {:?}", tup);

    // destructuring a tuple, with pattern matching
    let (a, b, c) = (1, 2.3, "four");
    println!("tuple part 0: {}", tup.0);
    println!("tuple part 0: {}", a);
    println!("tuple part 1: {}", b);
    println!("tuple part 2: {}", c);

    // array (allocated on the stack)
    let ar = [1, 2, 3, 4, 5];
    println!("array: {:?}", ar);

    let months: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    println!("months: {:?}", months);
    println!("month1: {}", months[1]);

    // zero'd array using alternate initializing
    let ar = [0; 10];
    println!("zero ar: {:?}", ar);

    // panic time
    // compile error:
    // println!("show item 33: {}", ar[33]);
    // compile error:
    // let index = 33;
    // println!("show item 33: {}", ar[index]);

    // The book says this should be a run time panic, but
    // it is caught at compile time
    // let a = [1, 2, 3, 4, 5];
    // let index = 33;
    // let element = a[index];
    // println!("show item 33: {}", element);
}
