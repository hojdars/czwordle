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

use crate::game::GameState;

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
}

struct Settings {
    word_length: u32,
    attempts: u32,
}

struct State<'d> {
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

async fn run_game(
    settings: &Settings,
    state: &mut State<'_>,
    font_params: &TextParams,
) -> ApplicationState {
    assert!(state.game.is_some());
    let game = state.game.as_mut().unwrap();
    assert!(matches!(game.get_game_state(), GameState::Ongoing { .. }));

    match handle_input(&mut state.word, settings.word_length).await {
        InputResult::Quit => {
            return ApplicationState::Quit;
        }
        InputResult::Entered => {
            match game.submit_guess(state.word.as_str()) {
                Ok(_) => {}
                Err(_) => {
                    println!("ERROR: incorrect word"); // TODO: GUI error message
                }
            }
            state.word.clear();
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

    ApplicationState::Game
}

async fn run_win(
    settings: &Settings,
    state: &mut State<'_>,
    font_params: &TextParams,
) -> ApplicationState {
    if state.game.is_none() {
        panic!("Game was empty, game cannot be empty if we are running it.");
    }

    if is_key_pressed(KeyCode::M) {
        get_char_pressed();
        return ApplicationState::Menu;
    }

    if is_key_pressed(KeyCode::Escape) {
        return ApplicationState::Quit;
    }

    clear_background(BLACK);

    draw_win(
        settings.word_length,
        state.game.as_ref().unwrap().get_guesses(),
        font_params,
    );

    ApplicationState::Game
}

async fn run_loss(
    settings: &Settings,
    state: &mut State<'_>,
    font_params: &TextParams,
) -> ApplicationState {
    if state.game.is_none() {
        panic!("Game was empty, game cannot be empty if we are running it.");
    }

    if is_key_pressed(KeyCode::M) {
        get_char_pressed();
        return ApplicationState::Menu;
    }

    if is_key_pressed(KeyCode::Escape) {
        return ApplicationState::Quit;
    }

    clear_background(BLACK);

    draw_loss(
        settings.word_length,
        state.game.as_ref().unwrap().get_guesses(),
        &state.game.as_ref().unwrap().get_correct_word(),
        font_params,
    );

    ApplicationState::Game
}

async fn menu_loop(settings: &mut Settings, font_params: &TextParams) -> ApplicationState {
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

        if state == ApplicationState::Game || state == ApplicationState::Quit {
            return state;
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
        word: String::new(),
        game: Some(Game::new(settings.attempts, dictionary)),
    };

    loop {
        let application_state = match state.game.as_ref().unwrap().get_game_state() {
            game::GameState::Ongoing(_) => run_game(settings, &mut state, font_params).await,
            game::GameState::Win(_) => run_win(settings, &mut state, font_params).await,
            game::GameState::Lose => run_loss(settings, &mut state, font_params).await,
        };

        if application_state != ApplicationState::Game {
            return application_state;
        }

        next_frame().await
    }
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
        let mut application_state = menu_loop(&mut settings, &font_params).await;

        if application_state == ApplicationState::Quit {
            break;
        }

        if dictionary.get_word_length() != settings.word_length {
            dictionary = Dictionary::new(text_file, settings.word_length);
        }

        application_state = game_loop(&settings, &dictionary, &font_params).await;

        if application_state == ApplicationState::Quit {
            break;
        }
    }
}
