fn main() {
    let s = String::from("hello there");
    let slen = calculate_length(&s);
    println!("string: '{}' is len {}", s, slen)
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
