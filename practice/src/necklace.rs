/// Daily Programmer `#383`: Determine if two strings describe the same necklace - if you can remove some number of letters from the beginning of one, attach them to the end in their original ordering, and get the other string.
///
/// # Examples
///
/// ```
/// assert_eq!(exercises::necklace::is_same("nicole", "icolen"), true);
/// assert_eq!(exercises::necklace::is_same("nicole", "lenico"), true);
/// assert_eq!(exercises::necklace::is_same("nicole", "coneli"), false);
/// assert_eq!(exercises::necklace::is_same("aabaaaaabaab", "aabaabaabaaa"), true);
/// assert_eq!(exercises::necklace::is_same("abc", "cba"), false);
/// assert_eq!(exercises::necklace::is_same("xxyyy", "xxxyy"), false);
/// assert_eq!(exercises::necklace::is_same("xyxxz", "xxyxz"), false);
/// assert_eq!(exercises::necklace::is_same("x", "x"), true);
/// assert_eq!(exercises::necklace::is_same("x", "xx"), false);
/// assert_eq!(exercises::necklace::is_same("x", ""), false);
/// assert_eq!(exercises::necklace::is_same("", ""), true);
/// ```
pub fn is_same(letters: &str, variation: &str) -> bool {
    if letters == "" {
        return true;
    }

    if letters.chars().count() != variation.chars().count() {
        return false;
    }

    let mut index = 0;
    let possible_variations: Vec<String> = letters
        .chars()
        .map(|_| {
            index += 1;
            let head = &letters[0..index];
            let tail = &letters[index..];
            format!("{}{}", tail, head)
        })
        .collect();

    possible_variations.contains(&variation.to_string())
}
