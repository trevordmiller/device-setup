/// Exercism `leap`: Given a year, report if it is a leap year.
///
/// # Examples
///
/// ```
/// assert_eq!(exercises::leap::is_leap_year(1996), true);
/// assert_eq!(exercises::leap::is_leap_year(1997), false);
/// assert_eq!(exercises::leap::is_leap_year(1900), false);
/// assert_eq!(exercises::leap::is_leap_year(2000), true);
/// assert_eq!(exercises::leap::is_leap_year(2400), true);
/// ```
pub fn is_leap_year(year: i64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
