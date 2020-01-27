pub fn abbreviate(phrase: &str) -> String {
    if phrase == "" {
        return phrase.to_string();
    }

    phrase
        .replace("_", "")
        .replace("-", " ")
        .split_whitespace()
        .map(|word| word.chars().next().unwrap().to_uppercase().to_string())
        .collect()
}
