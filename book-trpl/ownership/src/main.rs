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

    //---------------------------------------
    let x = 5;
    // note: this is a copy
    // (small, statically sized type on the stack)
    let y = x;
    // both are accessible
    println!("x: {}, y: {}", x, y);

    let a = String::from("boats");
    // note: this is a move not a copy
    // (heap/complex structure without a Copy trait
    let b = a;
    println!("b: {}", b);
    // This won't work because a is no longer valid
    // println!("a: {}", a)

    //---------------------------------------
}
