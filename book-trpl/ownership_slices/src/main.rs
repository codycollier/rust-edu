fn main() {
    let s = String::from("win the stage");
    let fw = end_of_first_word(&s);
    println!("fw: {}", fw)
}

fn end_of_first_word(s: &String) -> usize {
    let sbytes = s.as_bytes();
    for (i, &item) in sbytes.iter().enumerate() {
        if item == b' ' {
            // return the index of the end of the word
            return i;
        }
    }
    // no space? return index of final spot
    return s.len();
}
