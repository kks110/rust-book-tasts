use crate::helper;

pub fn run() {
    const VOWELS: [&str; 10] = [
        "A",
        "a",
        "E",
        "e",
        "I",
        "i",
        "O",
        "o",
        "U",
        "u",
    ];

    println!("Please enter a sentence: ");
    let input = helper::get_user_input();

    let split_input = input.split_whitespace();

    let mut new_sentence = String::from("");

    for word in split_input {
        let mut word_string = String::from(word);
        let first_letter = word_string.remove(0);
        let first_letter_string = first_letter.to_string();

        if VOWELS.contains(&&*first_letter_string) {
            let new_word = first_letter_string + &*word_string + "hay ";
            new_sentence.push_str(&*new_word);
        } else {
            let new_word = word_string + &*first_letter_string + "ay ";
            new_sentence.push_str(&*new_word);
        }
    }
    println!("{}", new_sentence)
}