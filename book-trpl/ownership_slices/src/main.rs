fn main() {
    // let mut s = String::from("win the stage");
    let s = String::from("win the stage");
    // let s = String::from("stage");

    let efw = end_of_first_word(&s);
    // this will clear s and efw is no longer correct
    // s.clear();
    println!("efw: {}", efw);

    let fw1 = first_word(&s);
    // now, by using a slice and true reference to s, we get an error
    // s.clear();
    println!("fw1: {}", fw1);

    let fw2 = first_word_efw(&s);
    println!("fw2: {}", fw2);
}

fn first_word(s: &String) -> &str {
    let sbytes = s.as_bytes();
    for (i, &item) in sbytes.iter().enumerate() {
        if item == b' ' {
            // return the index of the end of the word
            return &s[..i];
        }
    }
    // no space? return index of final spot
    &s[..]
}

fn first_word_efw(s: &String) -> &str {
    let efw = end_of_first_word(s);
    &s[..efw]
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
