use std::fs;
use std::io;
use std::io::Write;

mod dictionary;
use dictionary::Dictionary;

mod game;
use game::Game;
use game::GuessError;

mod letters;

mod printer;

pub fn run(path_to_dict: &str) {
    // TODO: Add three options:
    //      '-h' for help,
    //      '-t --tries' for altering the number of tries,
    //      '-l --length' for altering the length of the guessed word

    let dictionary = load_dictionary(path_to_dict);

    loop {
        // TODO: Allow playing more than one game without infinite loops
        println!("New game! Guess a word!");
        play_one_game(&dictionary);
    }
}

fn load_dictionary(path: &str) -> Dictionary {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    Dictionary::new(&contents)
}

fn play_one_game(dictionary: &Dictionary) {
    let mut game = Game::new(6, dictionary);

    loop {
        let guessed_word = input_word(game.get_remaining_guesses() + 1, game.get_total_guesses());
        let guess_result = game.submit_guess(&guessed_word);

        let mut is_game_over = false;
        match guess_result {
            Ok(_) => is_game_over = handle_valid_guess(&game),
            Err(error) => handle_guess_error(&guessed_word, &error),
        }

        if is_game_over {
            break;
        }
    }
}

fn input_word(guess_number: u32, available_guesses: u32) -> String {
    let mut input = String::new();
    printer::print_caret(guess_number, available_guesses);
    let _ = io::stdout().flush();

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_uppercase().to_owned(),
        Err(_) => panic!("Cannot read a line, exit."),
    }
}

fn handle_valid_guess(game: &Game) -> bool {
    printer::print_divider();
    printer::print_guesses(game.get_guesses());
    printer::print_divider();
    printer::print_letters(game.get_letters());

    match game.get_game_state() {
        game::GameState::Win(tries) => {
            printer::print_win(tries);
            return true;
        }
        game::GameState::Lose => {
            printer::print_lose(&game.get_correct_word());
            return true;
        }
        game::GameState::Ongoing(_) => {
            print!("\n");
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
