use std::fs;
use std::io;

extern crate colored;
use colored::*;

mod dictionary;
use dictionary::Dictionary;

mod game;
use game::Game;
use game::Guess;
use game::GuessError;

pub fn run(path_to_dict: &str) {
    let dictionary = setup(path_to_dict);

    loop {
        // TODO: Allow playing more than one game without infinite loops
        println!("New game! Guess a word!");
        play_one_game(&dictionary);
    }
}

fn setup(path_to_dict: &str) -> Dictionary {
    load_dictionary(path_to_dict)
}

fn play_one_game(dictionary: &Dictionary) {
    let mut game = Game::new(6, dictionary);

    loop {
        let guessed_word = input_word();
        let guess_result = game.submit_guess(&guessed_word);

        let mut is_game_over = false;
        match guess_result {
            Ok(guess) => is_game_over = handle_valid_guess(&game, &guess),
            Err(error) => handle_guess_error(&guessed_word, &error),
        }

        if is_game_over {
            break;
        }
    }
}

fn handle_valid_guess(game: &Game, guess: &Guess) -> bool {
    print_guess(&guess);

    match game.get_game_state() {
        game::GameState::Win(tries) => {
            print_win(tries);
            return true;
        }
        game::GameState::Lose => {
            print_lose(&game.get_correct_word());
            return true;
        }
        game::GameState::Ongoing(_) => {
            return false;
        }
    }
}

fn handle_guess_error(guessed_word: &str, error: &GuessError) {
    match error {
        GuessError::NotInDictionary => println!("'{}' is not in the dictionary!", guessed_word),
        GuessError::WrongLength(letter_count) => println!(
            "Word needs to be 5 letters long! Your word was {} letters long!",
            letter_count
        ),
    }
}

fn print_win(tries: u32) {
    println!(
        "Yay! {}",
        format!("You win in {} tries!", tries)
            .to_string()
            .green()
            .bold()
    );
}

fn print_lose(correct_word: &str) {
    println!(
        "You lose! :( The word was: {}",
        correct_word.to_string().red().bold()
    );
}

fn load_dictionary(path: &str) -> Dictionary {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    Dictionary::new(&contents)
}

fn input_word() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_uppercase().to_owned(),
        Err(_) => panic!("Cannot read a line, exit."),
    }
}

fn print_guess(guess: &Guess) {
    let mut i: u32 = 0;
    for char in guess.word.chars() {
        if guess.green_positions.contains(&i) {
            print!("{} ", char.to_string().to_uppercase().green().bold());
        } else if guess.yellow_positions.contains(&i) {
            print!("{} ", char.to_string().to_uppercase().yellow().bold());
        } else {
            print!("{} ", char.to_string().to_uppercase());
        }
        i = i + 1;
    }
    print!("\n");
}
