fn main() {
    println!("outer scope!");

    {
        println!("inner scope!");
        let s = "hello";
        println!("s: {}", s)
    }

    println!("back to outer scope!");

    // no longer in scope.  can't reference.
    // println!("s: {}", s)

    // new var and owner, back in scope
    let s = "hi again";
    println!("s: {}", s);

    //---------------------------------------

    let mut s = String::from("hello");
    println!("s: {}", s);
    // s += ", world again!";
    s.push_str(", world again!");
    println!("s: {}", s);
}
