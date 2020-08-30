fn main() {
    // let mut s = String::from("win the stage");
    let s = String::from("win the stage");
    // let s = String::from("stage");

    let efw = end_of_first_word(&s);
    // this will clear s and efw is no longer correct
    // s.clear();
    println!("efw: {}", efw);

    let fw1 = first_word_efw(&s);
    println!("fw1: {}", fw1);

    let fw2 = first_word(&s);
    // now, by using a slice and true reference to s, we get an error
    // s.clear();
    println!("fw2: {}", fw2);

    let fw3 = first_word_better(&s[..]);
    println!("fw3: {}", fw3);

    // &str is more flexible
    // Works for: string lits (which are just slices),
    // slices of string lits, and slices of String
    let string_lit = "win a race";
    let fw4 = first_word_better(string_lit);
    println!("fw4: {}", fw4);
    let fw5 = first_word_better(&string_lit[..]);
    println!("fw5: {}", fw5);

    // not just for strings
    // &[i32]
    let a = [1, 2, 3, 4, 5];
    // slice of &[i32]
    let b = &a[0..3];
    println!("a: {:?} b: {:?}", a, b)
}

fn first_word_better(s: &str) -> &str {
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
