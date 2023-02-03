use macroquad::{
    prelude::{get_char_pressed, is_key_pressed, is_key_released, KeyCode},
    text::TextParams,
    texture::Texture2D,
};

use crate::dictionary::Dictionary;

use crate::game::Game;
use crate::game::GameState;

use crate::gui::graphics::Graphics;
use crate::gui::menu::Menu;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ApplicationState {
    Menu,
    NewGame,
    Game,
    Quit,
}

#[derive(Debug, Copy, Clone)]
pub struct MainMenuData {
    pub state: ApplicationState,
    pub settings: Settings,
}

#[derive(Debug, Copy, Clone)]
pub struct Settings {
    pub word_length: u32,
    pub attempts: u32,
}

pub struct App<'s> {
    pub settings: Settings,

    gui: Graphics,
    text_file: &'s str,

    word: String,
}

enum InputResult {
    Incomplete,
    Entered,
    Quit,
}

impl<'s, 'd, 'm, 'n> App<'s> {
    pub fn new(
        text_file: &'s str,
        font: TextParams,
        logo: Texture2D,
        box_textures: Vec<Texture2D>,
    ) -> App<'s> {
        App {
            text_file,
            gui: Graphics::new(font, logo, box_textures),
            settings: Settings {
                word_length: 5,
                attempts: 6,
            },
            word: String::new(),
        }
    }

    pub fn make_dictionary(&self) -> Dictionary {
        Dictionary::new(self.text_file, self.settings.word_length)
    }

    pub async fn run_menu_loop(&mut self) -> ApplicationState {
        let mut main_menu = App::make_main_menu(self.settings.attempts, self.settings.word_length);
        loop {
            let y_start: f32 = self.gui.draw_menu_header();
            let result = main_menu.run(y_start, &mut self.gui);
            self.settings = result.settings;

            macroquad::window::next_frame().await;

            if result.state != ApplicationState::Menu {
                println!("return from menu loop, state={:?}", result.state);
                return result.state;
            }
        }
    }

    pub async fn run_game_loop(&mut self, dictionary: &'d mut Dictionary) -> ApplicationState {
        let mut game: Game = self.make_game(dictionary);
        println!("{}", game.get_correct_word());

        let mut game_over_menu = App::make_game_over_menu();

        loop {
            let app_state: ApplicationState = match game.get_game_state() {
                GameState::Ongoing(_) => self.run_game_frame(&mut game),
                GameState::Win(_) => self.run_win_frame(&mut game, &mut game_over_menu),
                GameState::Lose => self.run_loss_frame(&mut game, &mut game_over_menu),
            };

            macroquad::window::next_frame().await;

            if app_state != ApplicationState::Game {
                while get_char_pressed().is_some() {}
                return app_state;
            }
        }
    }

    fn make_main_menu(attempts: u32, word_length: u32) -> Menu<'m, MainMenuData> {
        let item_callback = |data: &mut MainMenuData, items: &Vec<String>| -> Vec<String> {
            let mut retval: Vec<String> = Vec::new();
            retval.push(items[0].to_string());
            retval.push(format!("{} {}", data.settings.attempts, items[1]));
            retval.push(format!("{} {}", data.settings.word_length, items[2]));
            retval.push(items[3].to_string());
            retval
        };

        let callback = |position: &mut u32, data: &mut MainMenuData| {
            if is_key_pressed(KeyCode::Enter) {
                match *position {
                    0 => data.state = ApplicationState::NewGame,
                    3 => data.state = ApplicationState::Quit,
                    _ => {}
                }
            } else if is_key_pressed(KeyCode::Escape) {
                data.state = ApplicationState::Quit;
            } else if is_key_pressed(KeyCode::Left) {
                match *position {
                    1 => data.settings.attempts -= 1,
                    2 => data.settings.word_length -= 1,
                    _ => {}
                }
            } else if is_key_pressed(KeyCode::Right) {
                match *position {
                    1 => data.settings.attempts += 1,
                    2 => data.settings.word_length += 1,
                    _ => {}
                }
            }
        };

        Menu::new_with_items_callback(
            Vec::from([
                "NEW GAME".to_string(),
                "ATTEMPTS".to_string(),
                "WORD LENGTH".to_string(),
                "QUIT".to_string(),
            ]),
            MainMenuData {
                state: ApplicationState::Menu,
                settings: Settings {
                    attempts,
                    word_length,
                },
            },
            callback,
            item_callback,
        )
    }

    fn make_game(&self, dictionary: &'d mut Dictionary) -> Game<'d> {
        if dictionary.get_word_length() != self.settings.word_length {
            *dictionary = Dictionary::new(self.text_file, self.settings.word_length)
        }

        Game::new(self.settings.attempts, dictionary)
    }

    fn make_game_over_menu() -> Menu<'n, ApplicationState> {
        let callback = |position: &mut u32, data: &mut ApplicationState| {
            if is_key_pressed(KeyCode::Enter) {
                match *position {
                    0 => *data = ApplicationState::NewGame,
                    1 => *data = ApplicationState::Menu,
                    _ => {}
                }
            } else if is_key_pressed(KeyCode::Escape) {
                *data = ApplicationState::Quit;
            }
        };

        Menu::new(
            Vec::from(["NEW GAME".to_string(), "MENU".to_string()]),
            ApplicationState::Game,
            callback,
        )
    }

    fn run_game_frame(&mut self, game: &mut Game) -> ApplicationState {
        assert!(matches!(game.get_game_state(), GameState::Ongoing { .. }));

        match self.handle_input() {
            InputResult::Quit => {
                return ApplicationState::Menu;
            }
            InputResult::Entered => {
                match game.submit_guess(self.word.as_str()) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("ERROR: incorrect word"); // TODO: GUI error message
                    }
                }
                self.word.clear();
            }
            InputResult::Incomplete => {}
        }

        self.gui.draw_game(&self.settings, game, &self.word);

        ApplicationState::Game
    }

    fn handle_input(&mut self) -> InputResult {
        if is_key_released(KeyCode::Escape) {
            return InputResult::Quit;
        }

        if is_key_pressed(KeyCode::Enter)
            && self.word.chars().count() == self.settings.word_length as usize
        {
            return InputResult::Entered;
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.word.pop();
            get_char_pressed();
        }

        while let Some(c) = get_char_pressed() {
            match c {
                '\u{00}'..='\u{1F}' => {
                    continue;
                }
                _ => {
                    if self.word.chars().count() < self.settings.word_length as usize {
                        for char in c.to_uppercase() {
                            self.word.push(char);
                        }
                    }
                }
            }
        }

        InputResult::Incomplete
    }

    fn run_win_frame(
        &mut self,
        game: &mut Game,
        menu: &mut Menu<ApplicationState>,
    ) -> ApplicationState {
        let y_start: f32 = self
            .gui
            .draw_win(self.settings.word_length, game.get_guesses());

        menu.run(y_start, &mut self.gui)
    }

    fn run_loss_frame(
        &mut self,
        game: &mut Game,
        menu: &mut Menu<ApplicationState>,
    ) -> ApplicationState {
        let y_start: f32 = self.gui.draw_loss(
            self.settings.word_length,
            game.get_guesses(),
            &game.get_correct_word(),
        );

        menu.run(y_start, &mut self.gui)
    }
}
