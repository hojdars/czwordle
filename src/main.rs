use macroquad::prelude::*;

mod gui;
use gui::draw_letters;
use gui::draw_loss;
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
#[derive(PartialEq)]
enum ApplicationState {
    Menu,
    Game,
    Quit,
    Win,
    Loss,
}

struct Settings {
    word_length: u32,
    attempts: u32,
}

struct State<'d> {
    game_state: ApplicationState,
    word: String,
    game: Option<Game<'d>>,
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

async fn run_game(settings: &Settings, state: &mut State<'_>, font_params: &TextParams) {
    if state.game.is_none() {
        panic!("Game was empty, game cannot be empty if we are running it.");
    }
    let game = state.game.as_mut().unwrap();

    if game.get_remaining_guesses() == 0 {
        state.game_state = ApplicationState::Loss;
        return;
    }

    match handle_input(&mut state.word, settings.word_length).await {
        InputResult::Quit => {
            state.game_state = ApplicationState::Quit;
            return;
        }
        InputResult::Entered => {
            let result = game.submit_guess(state.word.as_str());
            match result {
                Ok(Guess { is_correct, .. }) => {
                    if is_correct {
                        state.game_state = ApplicationState::Win;
                    }
                    state.word.clear();
                }
                Err(GuessError::NotInDictionary) => {
                    state.word.clear();
                }
                Err(GuessError::WrongLength(_len)) => {
                    state.word.clear();
                }
            }
        }
        InputResult::Incomplete => {}
    }

    clear_background(BLACK);

    draw_words(
        settings.word_length,
        &state.word,
        game.get_guesses(),
        font_params,
    );

    draw_letters(game.get_letters(), game.get_total_guesses(), font_params);
}

async fn run_win(settings: &Settings, state: &mut State<'_>, font_params: &TextParams) {
    if state.game.is_none() {
        panic!("Game was empty, game cannot be empty if we are running it.");
    }

    if is_key_pressed(KeyCode::M) {
        state.game_state = ApplicationState::Menu;
        get_char_pressed();
        return;
    }

    if is_key_pressed(KeyCode::Escape) {
        state.game_state = ApplicationState::Quit;
        return;
    }

    clear_background(BLACK);

    draw_win(
        settings.word_length,
        state.game.as_ref().unwrap().get_guesses(),
        font_params,
    );
}

async fn run_loss(settings: &Settings, state: &mut State<'_>, font_params: &TextParams) {
    if state.game.is_none() {
        panic!("Game was empty, game cannot be empty if we are running it.");
    }

    if is_key_pressed(KeyCode::M) {
        state.game_state = ApplicationState::Menu;
        get_char_pressed();
        return;
    }

    if is_key_pressed(KeyCode::Escape) {
        state.game_state = ApplicationState::Quit;
        return;
    }

    clear_background(BLACK);

    draw_loss(
        settings.word_length,
        state.game.as_ref().unwrap().get_guesses(),
        &state.game.as_ref().unwrap().get_correct_word(),
        font_params,
    );
}

async fn menu_loop(
    settings: &mut Settings,
    dictionary: &mut Dictionary,
    font_params: &TextParams,
    text_file: &str,
) -> bool {
    let mut state: ApplicationState = ApplicationState::Menu;
    loop {
        if is_key_pressed(KeyCode::N) {
            state = ApplicationState::Game;
            while get_char_pressed().is_some() {}
        } else if is_key_released(KeyCode::Up) {
            settings.attempts += 1;
        } else if is_key_released(KeyCode::Down) && settings.attempts > 1 {
            settings.attempts -= 1;
        } else if is_key_released(KeyCode::Left) && settings.word_length > 2 {
            settings.word_length -= 1;
        } else if is_key_released(KeyCode::Right) && settings.word_length < 8 {
            settings.word_length += 1;
        } else if is_key_pressed(KeyCode::Escape) {
            state = ApplicationState::Quit;
        }

        draw_menu(settings.attempts, settings.word_length, font_params);

        if state == ApplicationState::Game {
            if dictionary.get_word_length() != settings.word_length {
                *dictionary = Dictionary::new(text_file, settings.word_length);
            }
            return true;
        }

        if state == ApplicationState::Quit {
            return false;
        }

        next_frame().await
    }
}

async fn game_loop(
    settings: &Settings,
    dictionary: &Dictionary,
    font_params: &TextParams,
) -> ApplicationState {
    let mut state: State = State {
        game_state: ApplicationState::Game,
        word: String::new(),
        game: Some(Game::new(settings.attempts, dictionary)),
    };

    loop {
        match state.game_state {
            ApplicationState::Menu => {
                break;
            }
            ApplicationState::Game => {
                run_game(settings, &mut state, font_params).await;
            }
            ApplicationState::Quit => {
                break;
            }
            ApplicationState::Win => {
                run_win(settings, &mut state, font_params).await;
            }
            ApplicationState::Loss => {
                run_loss(settings, &mut state, font_params).await;
            }
        }

        next_frame().await
    }
    state.game_state
}

fn window_conf() -> Conf {
    Conf {
        window_title: "czWORDLE".to_owned(),
        fullscreen: false,
        window_height: 1000,
        window_width: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_params: TextParams = load_fonts("ttf/NotoSansMono-Regular.ttf").await;

    let text_file = std::include_str!("../data/jmena.txt");

    let mut settings: Settings = Settings {
        word_length: 5,
        attempts: 6,
    };

    let mut dictionary: Dictionary = Dictionary::new(text_file, settings.word_length);

    loop {
        let c = menu_loop(&mut settings, &mut dictionary, &font_params, text_file).await;
        if !c {
            break;
        }

        let r = game_loop(&settings, &dictionary, &font_params).await;
        if r == ApplicationState::Quit {
            break;
        }
    }
}
