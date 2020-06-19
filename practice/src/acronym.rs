/// Exercism `acronym`: Convert a phrase to its acronym.
///
/// # Examples
///
/// ```
/// assert_eq!(practice::acronym::abbreviate("Portable Network Graphics"), "PNG");
/// assert_eq!(practice::acronym::abbreviate("Ruby on Rails"), "ROR");
/// assert_eq!(practice::acronym::abbreviate("First In, First Out"), "FIFO");
/// assert_eq!(practice::acronym::abbreviate("GNU Image Manipulation Program"), "GIMP");
/// assert_eq!(practice::acronym::abbreviate("Complementary metal-oxide semiconductor"), "CMOS");
/// assert_eq!(practice::acronym::abbreviate("Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"), "ROTFLSHTMDCOALM");
/// assert_eq!(practice::acronym::abbreviate("Something - I made up from thin air"), "SIMUFTA");
/// assert_eq!(practice::acronym::abbreviate("Halley's Comet"), "HC");
/// assert_eq!(practice::acronym::abbreviate("The Road _Not_ Taken"), "TRNT");
/// assert_eq!(practice::acronym::abbreviate(""), "");
/// ```
pub fn abbreviate(phrase: &str) -> String {
    if phrase == "" {
        return phrase.to_string();
    }

    phrase
        .replace("_", "")
        .replace("-", " ")
        .split_whitespace()
        .map(|word| match word.chars().next() {
            Some(initial) => initial.to_uppercase().to_string(),
            None => "".to_string(),
        })
        .collect()
}
