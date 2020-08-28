fn main() {
    let s = String::from("hello there");
    let slen = calculate_length(&s);
    println!("string: '{}' is len {}", s, slen);

    // mutable variable and mutable reference
    let mut s2 = String::from("hello there");
    change(&mut s2);
    println!("s2: '{}'", s);

    // compiler preventions:
    // cannot have two mutable refs at once
    // let ref1 = &mut s2;
    // let ref2 = &mut s2;
    // err: second mutable borrow occurs here ^^^
    // println!("ref1: '{}'  ref2: '{}'", ref1, ref2);

    // compiler preventions:
    // cannot have a mutable ref (writer) when there is an immutable ref (reader)
    // another data race prevention tactic
    // let ref1 = &s2;
    // let ref2 = &mut s2;
    // println!("ref1: '{}'  ref2: '{}'", ref1, ref2);

    // compiler preventions:
    // but it is fine if the mutable ref comes after the immutable ref is no longer used and out of scope
    let ref1 = &s2;
    println!("ref1: '{}'", ref1);
    let ref2 = &mut s2;
    println!("ref2: '{}'", ref2);
}

fn calculate_length(st: &String) -> usize {
    // cannot change the value at the reference or assign to the immutable arg
    // st = &String::from("foo");
    // err: cannot assign to immutable argument

    // said differently, due to the reference, st is merely borrowed not owned, so it can't be changed here

    // cannot do this more direct change either
    // st.push_str(", how are you");
    // err: ^^ `st` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    st.len()
} // st reference goes out of scope, but with no ownership of value, which means no drop

fn change(st: &mut String) {
    st.push_str(", how are you")
}
