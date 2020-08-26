fn main() {
    let s = String::from("hello");
    take_ownership(s);
    // can't do this because the value moved inside take_ownership
    // println!("in main scope. s: {}", s);

    let i = 42;
    makes_copy(i);
    // can do this. int is on the stack and thus copied inherently.
    println!("in main scope. i: {}", i)
}

fn take_ownership(s: String) {
    println!("inside take_ownership. s: {}", s);
}

fn makes_copy(i: i32) {
    println!("inside makes_copy. i: {}", i)
}
