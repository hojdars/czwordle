use crate::game::GameState;

use super::dictionary::Dictionary;
use super::Game;
use super::Guess;
use super::GuessError;

#[test]
fn game_creation() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nmicha/OK";
    let d = Dictionary::new(words, 5);

    let game = Game::new(6, &d);

    assert_eq!(game.state.maximum_tries, 6);
    assert_eq!(game.state.word_to_guess.chars().count(), 5);
    assert_eq!(game.state.guesses.len(), 0);
}

#[test]
fn get_game_state_after_all_guesses_are_depleted() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nmicha/OK";
    let d = Dictionary::new(words, 5);
    let mut game = Game::new(6, &d);

    let state = game.get_game_state();
    assert!(state == GameState::Ongoing(0));

    game.state.guesses.push(Guess {
        is_correct: false,
        word: "TEST".to_string(),
        yellow_positions: Vec::new(),
        green_positions: Vec::new(),
    });

    let state = game.get_game_state();
    assert!(state == GameState::Ongoing(1));

    for _ in 0..5 {
        game.state.guesses.push(Guess {
            is_correct: false,
            word: "TEST".to_string(),
            yellow_positions: Vec::new(),
            green_positions: Vec::new(),
        });
    }

    let state = game.get_game_state();
    assert!(state == GameState::Lose);
}

#[test]
fn get_game_state_after_correct_guess() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nmicha/OK";
    let d = Dictionary::new(words, 5);
    let mut game = Game::new(6, &d);

    let state = game.get_game_state();
    assert!(state == GameState::Ongoing(0));

    game.state.guesses.push(Guess {
        is_correct: true,
        word: game.state.word_to_guess.clone(),
        yellow_positions: Vec::new(),
        green_positions: Vec::new(),
    });

    let state = game.get_game_state();
    assert!(state == GameState::Win(1));
}

#[test]
fn get_correct_word() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK";
    let d = Dictionary::new(words, 5);
    let game = Game::new(6, &d);

    assert_eq!(game.get_correct_word(), "CIVKA");
}

#[test]
fn submit_guess_guess_is_correct() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK";
    let d = Dictionary::new(words, 5);
    let mut game = Game::new(6, &d);

    let error_length = game.submit_guess("guessed_word");
    assert!(error_length.is_err());
    assert_eq!(error_length.err().unwrap(), GuessError::WrongLength(12));

    let error_not_in_dict = game.submit_guess("abcde");
    assert!(error_not_in_dict.is_err());
    assert_eq!(
        error_not_in_dict.err().unwrap(),
        GuessError::NotInDictionary
    );

    let works = game.submit_guess("civka");
    assert!(works.is_ok());
    let g = works.unwrap();
    assert_eq!(g.word, "CIVKA");
    assert_eq!(g.green_positions.len(), 5);
    assert!(g.is_correct);
    assert_eq!(game.state.guesses.len(), 1);
}

#[test]
fn submit_guess_guess_is_not_correct() {
    let words = "pivo/SHORT\nauto/SHORT\ncivka/OK\nxyzya/OK";
    let d = Dictionary::new(words, 5);
    let mut game = Game::new(6, &d);
    game.state.word_to_guess = "civka".to_uppercase();

    let works = game.submit_guess("xyzya");
    assert!(works.is_ok());
    let g = works.unwrap();
    assert_eq!(g.word, "XYZYA");
    assert_eq!(g.green_positions.len(), 1); // only 'A' at the end is correect
    assert!(!g.is_correct);
}
