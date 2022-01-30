use std::fs;
use std::io;

mod dictionary;
use dictionary::Dictionary;

mod game;
use game::Game;
use game::GuessError;

mod letters;

mod printer;

enum ProgramFlow {
    NextRound,
    Exit,
}

pub fn run(path_to_dict: &str) {
    // TODO: Add three options:
    //      '-h' for help,
    //      '-t --tries' for altering the number of tries,
    //      '-l --length' for altering the length of the guessed word

    let dictionary = load_dictionary(path_to_dict);
    play_one_game(&dictionary);

    loop {
        match get_next_action() {
            ProgramFlow::NextRound => {
                printer::clear_screen();
                play_one_game(&dictionary);
            }
            ProgramFlow::Exit => break,
        };
    }
}

fn load_dictionary(path: &str) -> Dictionary {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    Dictionary::new(&contents)
}

fn play_one_game(dictionary: &Dictionary) {
    let mut game = Game::new(6, dictionary);

    let mut last_error: Option<GuessError> = None;
    loop {
        printer::clear_screen();

        if !game.get_guesses().is_empty() {
            printer::print_guesses(game.get_guesses());
            printer::print_divider();
            printer::print_letters(game.get_letters());
        }

        if let Some(err) = last_error {
            handle_guess_error(&err);
            last_error = None;
        }

        let guessed_word = input_word(game.get_remaining_guesses() + 1, game.get_total_guesses());
        let guess_result = game.submit_guess(&guessed_word);

        let mut is_game_over = false;
        match guess_result {
            Ok(_) => is_game_over = handle_valid_guess(&game),
            Err(error) => last_error = Some(error),
        }

        if is_game_over {
            break;
        }
    }
}

fn get_next_action() -> ProgramFlow {
    printer::print_new_game_query();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.chars().count() == 0 {
                    return ProgramFlow::NextRound;
                }

                if input.chars().count() > 1 {
                    continue;
                }

                let char: char = input.chars().next().unwrap();
                if char == 'y' || char == 'Y' {
                    return ProgramFlow::NextRound;
                } else if char == 'n' || char == 'N' {
                    return ProgramFlow::Exit;
                } else {
                    continue;
                }
            }
            Err(_) => panic!("Cannot read a line, exit."),
        }
    }
}

fn input_word(guess_number: u32, available_guesses: u32) -> String {
    let mut input = String::new();
    printer::print_caret(guess_number, available_guesses);

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_uppercase().to_owned(),
        Err(_) => panic!("Cannot read a line, exit."),
    }
}

fn handle_valid_guess(game: &Game) -> bool {
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

fn handle_guess_error(error: &GuessError) {
    match error {
        GuessError::NotInDictionary => println!("Word not in the dictionary!"),
        GuessError::WrongLength(letter_count) => println!(
            "Word needs to be 5 letters long! Your word was {} letters long!",
            letter_count
        ),
    }
}
