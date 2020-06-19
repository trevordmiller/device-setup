/// Daily Programmer `#381`: Given a Yahtzee dice roll, represented as a sorted list of 5 integers, each of which is between 1 and 6 inclusive, find the maximum possible score for this roll.
///
/// # Examples
///
/// ```
/// assert_eq!(practice::yahtzee::calculate_maxium_score(vec![2, 3, 5, 5, 6]), 10);
/// assert_eq!(practice::yahtzee::calculate_maxium_score(vec![1, 1, 1, 1, 3]), 4);
/// assert_eq!(practice::yahtzee::calculate_maxium_score(vec![1, 1, 1, 3, 3]), 6);
/// assert_eq!(practice::yahtzee::calculate_maxium_score(vec![1, 2, 3, 4, 5]), 5);
/// assert_eq!(practice::yahtzee::calculate_maxium_score(vec![6, 6, 6, 6, 6]), 30);
/// ```
pub fn calculate_maxium_score(roll: Vec<usize>) -> usize {
    fn calculate_possible_score(roll: &[usize], number: usize) -> usize {
        let amount_of_number = roll.iter().filter(|dice| dice == &&number).count();

        amount_of_number * number
    }

    let possible_scores = vec![
        calculate_possible_score(&roll, 1),
        calculate_possible_score(&roll, 2),
        calculate_possible_score(&roll, 3),
        calculate_possible_score(&roll, 4),
        calculate_possible_score(&roll, 5),
        calculate_possible_score(&roll, 6),
    ];

    match possible_scores.iter().max() {
        Some(maximum_score) => *maximum_score,
        None => 0,
    }
}
