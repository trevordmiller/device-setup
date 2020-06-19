/// Exercism `anagram`: Given a word and a list of possible anagrams, select the correct sublist.
///
/// # Examples
///
/// ```
/// let empty: Vec<&str> = vec![];
/// assert_eq!(practice::anagram::filter_anagrams("diaper", &["hello", "world", "zombies", "pants"]), empty);
/// assert_eq!(practice::anagram::filter_anagrams("master", &["stream", "pigeon", "maters"]), &["stream", "maters"]);
/// assert_eq!(practice::anagram::filter_anagrams("good", &["dog", "goody"]), empty);
/// assert_eq!(practice::anagram::filter_anagrams("listen", &["enlists", "google", "inlets", "banana"]), &["inlets"]);
/// assert_eq!(practice::anagram::filter_anagrams("allergy", &["gallery", "ballerina", "regally", "clergy", "largely", "leading"]), &["gallery", "regally", "largely"]);
/// assert_eq!(practice::anagram::filter_anagrams("nose", &["Eons", "ONES"]), &["Eons", "ONES"]);
/// assert_eq!(practice::anagram::filter_anagrams("mass", &["last"]), empty);
/// assert_eq!(practice::anagram::filter_anagrams("Orchestra", &["cashregister", "Carthorse", "radishes"]), &["Carthorse"]);
/// assert_eq!(practice::anagram::filter_anagrams("Orchestra", &["cashregister", "carthorse", "radishes"]), &["carthorse"]);
/// assert_eq!(practice::anagram::filter_anagrams("orchestra", &["cashregister", "Carthorse", "radishes"]), &["Carthorse"]);
/// assert_eq!(practice::anagram::filter_anagrams("go", &["go Go GO"]), empty);
/// assert_eq!(practice::anagram::filter_anagrams("tapper", &["patter"]), empty);
/// assert_eq!(practice::anagram::filter_anagrams("BANANA", &["BANANA", "Banana", "banana"]), empty);
/// assert_eq!(practice::anagram::filter_anagrams("LISTEN", &["Listen", "Silent", "LISTEN"]), &["Silent"]);
/// ```
pub fn filter_anagrams<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> Vec<&'a str> {
    fn sort(word: &str) -> Vec<char> {
        let mut sorted_word: Vec<char> = word.chars().collect();
        sorted_word.sort();
        sorted_word
    }

    let word = word.to_lowercase();
    let sorted_word = sort(&word);

    possible_anagrams
        .iter()
        .cloned()
        .filter(|&possible_anagram| {
            let possible_anagram = possible_anagram.to_lowercase();
            sort(&possible_anagram) == sorted_word && possible_anagram != word
        })
        .collect::<Vec<&'a str>>()
}
