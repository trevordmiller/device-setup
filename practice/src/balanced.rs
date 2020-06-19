/// Daily Programmer `#372`: Given a string containing only the characters 'x' and 'y', find whether there are the same number of 'x's and 'y's.
///
/// # Examples
///
/// ```
/// assert_eq!(practice::balanced::is_balanced("xxxyyy"), true);
/// assert_eq!(practice::balanced::is_balanced("yyyxxx"), true);
/// assert_eq!(practice::balanced::is_balanced("xxxyyyy"), false);
/// assert_eq!(practice::balanced::is_balanced("yyxyxxyxxyyyyxxxyxyx"), true);
/// assert_eq!(practice::balanced::is_balanced("xyxxxxyyyxyxxyxxyy"), false);
/// assert_eq!(practice::balanced::is_balanced(""), true);
/// assert_eq!(practice::balanced::is_balanced("x"), false);
/// ```
pub fn is_balanced(letters: &str) -> bool {
    let size = letters.chars().count();

    match size {
        0 => true,
        1 => false,
        _ => {
            let x_count = letters.chars().filter(|letter| letter.eq(&'x')).count();
            let y_count = letters.chars().filter(|letter| letter.eq(&'y')).count();

            x_count == y_count
        }
    }
}
