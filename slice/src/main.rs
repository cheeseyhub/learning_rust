fn get_sentence() -> String {
    let mut sentence = String::new();
    std::io::stdin()
        .read_line(&mut sentence)
        .expect("Error getting input");
    return sentence;
}
fn get_first_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[..i];
        }
    }
    &sentence[..]
}

fn main() {
    let sentence = get_sentence();
    let word = get_first_word(&sentence);

    let array = [1, 2, 3, 4];
    let array_slice = &array[1..=3];
    println!("The first word is {word}");
    println!("{:?}", array_slice);
}
