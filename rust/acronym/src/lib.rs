pub fn abbreviate(phrase: &str) -> String {
    let words: Vec<&str> = phrase
        .split(|c: char| c != '\'' && !c.is_alphabetic())
        .collect();
    let first_letters: String = words
        .iter()
        .filter(|word| word.len() > 0)
        .map(|word| word.chars().next().unwrap())
        .collect::<String>()
        .to_uppercase();

    return first_letters;
}
