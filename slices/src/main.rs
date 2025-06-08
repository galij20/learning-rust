fn main() {
    let mut s = String::from("It is LSPPDay8.");
    let word = first_word(&s);

    println!("The first word of {s} is {word}");

    
    let literal = "Hello, World!";
    
    let word = first_word(literal);
    println!("The first word of {literal} is {word}");

    s.clear();
}

fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
