fn main() {
    let s = String::from("hello there");
    let slen = calculate_length(&s);
    println!("string: '{}' is len {}", s, slen)
}

fn calculate_length(st: &String) -> usize {
    // cannot assign to immutable argument
    // st = &String::from("foo");
    st.len()
} // st reference goes out of scope, but with no ownership of value, which means no drop
