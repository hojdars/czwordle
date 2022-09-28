use macroquad::prelude::*;

mod gui;
use gui::draw_guesses;
use gui::draw_menu;
use gui::draw_win;
use gui::draw_words;

mod dictionary;
use dictionary::Dictionary;

mod letters;

mod game;
use game::Game;
use game::Guess;
use game::GuessError;

enum InputResult {
    Incomplete,
    Entered,
    Quit,
}

enum GameState {
    Menu,
    Game,
    Quit,
    Win,
}

struct State<'d> {
    game_state: GameState,
    word: String,
    game: Game<'d>,
    tries: u32,
    word_length: u32,
}

async fn load_fonts(path: &str) -> TextParams {
    let pf = load_ttf_font(path).await;
    let poppins_font = pf.unwrap();

    TextParams {
        font_size: 42,
        font: poppins_font,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: Color::new(1.0, 1.0, 0.0, 1.0),
    }
}

async fn handle_input(word: &mut String, max_len: u32) -> InputResult {
    if is_key_pressed(KeyCode::Escape) {
        return InputResult::Quit;
    }

    if is_key_pressed(KeyCode::Enter) && word.chars().count() == max_len as usize {
        return InputResult::Entered;
    }

    if is_key_pressed(KeyCode::Backspace) {
        word.pop();
        get_char_pressed();
    }

    while let Some(c) = get_char_pressed() {
        match c {
            '\u{00}'..='\u{1F}' => {
                continue;
            }
            _ => {
                if word.chars().count() < max_len as usize {
                    for char in c.to_uppercase() {
                        word.push(char);
                    }
                }
            }
        }
    }

    InputResult::Incomplete
}

async fn run_menu<'a, 'b>(state: &mut State<'a>, dict: &'b Dictionary, font_params: &TextParams)
where
    'b: 'a,
{
    if is_key_pressed(KeyCode::N) {
        state.game_state = GameState::Game;
        while let Some(_) = get_char_pressed() {}
        state.game = Game::new(state.tries, dict);
        return;
    }

    if is_key_released(KeyCode::Up) {
        state.tries += 1;
    }

    if is_key_released(KeyCode::Down) {
        state.tries -= 1;
    }
    if is_key_pressed(KeyCode::Escape) {
        state.game_state = GameState::Quit;
        return;
    }

    draw_menu(state.tries, font_params);
}

async fn run_game(state: &mut State<'_>, font_params: &TextParams) {
    if state.game.get_remaining_guesses() == 0 {
        state.game_state = GameState::Menu; //TODO: Lose
        return;
    }

    match handle_input(&mut state.word, state.word_length).await {
        InputResult::Quit => {
            state.game_state = GameState::Quit;
            return;
        }
        InputResult::Entered => {
            let result = state.game.submit_guess(state.word.as_str());
            match result {
                Ok(Guess { is_correct, .. }) => {
                    if is_correct {
                        state.game_state = GameState::Win;
                    }
                    state.word.clear();
                }
                Err(GuessError::NotInDictionary) => {
                    state.word.clear();
                }
                Err(GuessError::WrongLength(len)) => {
                    state.word.clear();
                }
            }
        }
        InputResult::Incomplete => {}
    }

    clear_background(BLACK);

    draw_words(
        state.word_length,
        &state.word,
        state.game.get_guesses(),
        &font_params,
    );
}

async fn run_win(state: &mut State<'_>, font_params: &TextParams) {
    if is_key_pressed(KeyCode::M) {
        state.game_state = GameState::Menu;
        get_char_pressed();
        return;
    }

    if is_key_pressed(KeyCode::Escape) {
        state.game_state = GameState::Quit;
        return;
    }

    clear_background(BLACK);

    draw_win(state.word_length, state.game.get_guesses(), &font_params);
}

#[macroquad::main("CZWORDLE")]
async fn main() {
    let font_params: TextParams = load_fonts("ttf/NotoSansMono-Regular.ttf").await;

    let text_file = std::include_str!("../data/jmena.txt");

    let dictionary: Dictionary = Dictionary::new(text_file, 5);

    let mut state: State = State {
        game_state: GameState::Menu,
        word: String::new(),
        game: Game::new(6, &dictionary),
        tries: 6,
        word_length: 5,
    };

    loop {
        match state.game_state {
            GameState::Menu => {
                run_menu(&mut state, &dictionary, &font_params).await;
            }
            GameState::Game => {
                run_game(&mut state, &font_params).await;
            }
            GameState::Quit => {
                break;
            }
            GameState::Win => {
                run_win(&mut state, &font_params).await;
            }
        }

        next_frame().await
    }
}
