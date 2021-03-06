use super::Dictionary;

#[test]
fn dictionary_creation() {
    let word_length = 5;

    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nmicha/OK";
    let d = Dictionary::new(words, word_length);
    assert_eq!(d.wordlist.len(), 2);
    assert_eq!(d.wordset.len(), 2);

    assert_eq!(
        d.wordlist.iter().next().unwrap().chars().count(),
        word_length.try_into().unwrap()
    );
}

#[test]
fn get_random_word() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nmicha/OK";
    let d = Dictionary::new(words, 5);

    let mut r = d.get_random_word();
    assert!(r == "civka".to_uppercase() || r == "micha".to_uppercase());

    r = d.get_random_word();
    assert!(r == "civka".to_uppercase() || r == "micha".to_uppercase());
}

#[test]
fn contains() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nmicha/OK";
    let d = Dictionary::new(words, 5);

    assert!(!d.contains("word"));
    assert!(!d.contains("wordle"));

    assert!(d.contains("civka"));
    assert!(d.contains("micha"));
}
