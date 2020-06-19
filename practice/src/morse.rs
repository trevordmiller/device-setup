/// Daily Programmer `#380`: Combine all the letters together into a single string consisting of only morse code dashes and dots.
///
/// # Examples
///
/// ```
/// assert_eq!(practice::morse::convert_letters("sos"), "...---...");
/// assert_eq!(practice::morse::convert_letters("daily"), "-...-...-..-.--");
/// assert_eq!(practice::morse::convert_letters("programmer"), ".--..-.-----..-..-----..-.");
/// assert_eq!(practice::morse::convert_letters("bits"), "-.....-...");
/// assert_eq!(practice::morse::convert_letters("three"), "-.....-...");
/// ```
pub fn convert_letters(word: &str) -> String {
    word.chars()
        .map(|letter| match letter {
            'a' => ".-",
            'b' => "-...",
            'c' => "-.-.",
            'd' => "-..",
            'e' => ".",
            'f' => "..-.",
            'g' => "--.",
            'h' => "....",
            'i' => "..",
            'j' => ".---",
            'k' => "-.-",
            'l' => ".-..",
            'm' => "--",
            'n' => "-.",
            'o' => "---",
            'p' => ".--.",
            'q' => "--.-",
            'r' => ".-.",
            's' => "...",
            't' => "-",
            'u' => "..-",
            'v' => "...-",
            'w' => ".--",
            'x' => "-..-",
            'y' => "-.--",
            'z' => "--..",
            _ => "",
        })
        .collect()
}
