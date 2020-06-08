/// Exercism `acronym`: Convert a phrase to its acronym.
///
/// # Examples
///
/// ```
/// assert_eq!(exercises::acronym::abbreviate("Portable Network Graphics"), "PNG");
/// assert_eq!(exercises::acronym::abbreviate("Ruby on Rails"), "ROR");
/// assert_eq!(exercises::acronym::abbreviate("First In, First Out"), "FIFO");
/// assert_eq!(exercises::acronym::abbreviate("GNU Image Manipulation Program"), "GIMP");
/// assert_eq!(exercises::acronym::abbreviate("Complementary metal-oxide semiconductor"), "CMOS");
/// assert_eq!(exercises::acronym::abbreviate("Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"), "ROTFLSHTMDCOALM");
/// assert_eq!(exercises::acronym::abbreviate("Something - I made up from thin air"), "SIMUFTA");
/// assert_eq!(exercises::acronym::abbreviate("Halley's Comet"), "HC");
/// assert_eq!(exercises::acronym::abbreviate("The Road _Not_ Taken"), "TRNT");
/// assert_eq!(exercises::acronym::abbreviate(""), "");
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
