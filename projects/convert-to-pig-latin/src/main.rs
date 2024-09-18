/* Task:
Convert strings to pig latin. The first consonant of each word
is moved to the end of the word and ay is added, so first becomes
irst-fay. Words that start with a vowel have hay added to the end
instead (apple becomes apple-hay). Keep in mind the details about
UTF-8 encoding!
*/

// ===========================================================================

const VOWEL_CHARS: [char; 5] = ['a', 'o', 'u', 'e', 'i'];

// ===========================================================================

fn main() {
    // Tests
    assert_eq!(convert_word_to_pig_latin("Latin"), "atin-Lay");
    assert_eq!(convert_word_to_pig_latin("Pig"), "ig-Pay");
    assert_eq!(convert_word_to_pig_latin("Apple"), "Apple-hay");

    assert_eq!(to_pig_latin("Pig Latin"), "ig-Pay atin-Lay");
    assert_eq!(
        to_pig_latin("Apple Cookies Watermelon Avocado"),
        "Apple-hay ookies-Cay atermelon-Way Avocado-hay"
    );
    //
}

fn convert_word_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        Some(c) if is_vowel(&c) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", chars.as_str(), c),
        None => String::new(),
    }
}

fn is_vowel(c: &char) -> bool {
    let lower_case_char: char = c.to_lowercase().next().unwrap();
    VOWEL_CHARS.contains(&lower_case_char)
}

fn to_pig_latin(text: &str) -> String {
    let mut pigged = String::from(text);
    for i in text.split_whitespace() {
        pigged = pigged.replace(i, &convert_word_to_pig_latin(i));
    }
    pigged
}
