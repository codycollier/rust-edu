fn main() {
    let s = String::from("hello");
    take_ownership(s);
    // can't do this because the value moved inside take_ownership
    // println!("in main scope. s: {}", s);

    let i = 42;
    makes_copy(i);
    // can do this. int is on the stack and thus copied inherently.
    println!("in main scope. i: {}", i);

    let foo = gives_ownership();
    println!("in main scope: foo: {}", foo);
    let baz = takes_and_returns_ownership(foo);
    // foo is gone
    // println!("in main scope: foo: {}", foo);
    // baz rules the day
    println!("in main scope: baz: {}", baz);

    // tuple returns on explicit passing and moves
    let phrase = String::from("hello from main");
    let (phrase_len, phrase) = calc_length(phrase);
    println!("The length of {} is {}", phrase, phrase_len);
}

fn take_ownership(s: String) {
    println!("inside take_ownership. s: {}", s);
} // don't forget an s drop happens here as the scope ends

fn makes_copy(i: i32) {
    println!("inside makes_copy. i: {}", i)
}

fn gives_ownership() -> String {
    let s2 = String::from("emerges");
    s2
}

fn takes_and_returns_ownership(s: String) -> String {
    return s;
}

fn calc_length(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}
