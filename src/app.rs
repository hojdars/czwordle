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

impl<'s, 'd> App<'s> {
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

    pub fn make_game(&self, dictionary: &'d mut Dictionary) -> Game<'d> {
        if dictionary.get_word_length() != self.settings.word_length {
            *dictionary = Dictionary::new(self.text_file, self.settings.word_length)
        }

        Game::new(self.settings.attempts, dictionary)
    }

    pub fn run_menu(&mut self, menu: &mut Menu<MainMenuData>) -> ApplicationState {
        let y_start: f32 = self.gui.draw_menu_header();

        menu.run(y_start, &mut self.gui).state
    }

    pub fn run_game(&mut self, game: &mut Game) -> ApplicationState {
        match game.get_game_state() {
            GameState::Ongoing(_) => self.run_game_frame(game),
            GameState::Win(_) => self.run_win_frame(game),
            GameState::Lose => self.run_loss_frame(game),
        }
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

    fn run_win_frame(&self, game: &mut Game) -> ApplicationState {
        if is_key_pressed(KeyCode::M) {
            get_char_pressed();
            return ApplicationState::Menu;
        }

        if is_key_pressed(KeyCode::N) {
            get_char_pressed();
            return ApplicationState::NewGame;
        }

        if is_key_pressed(KeyCode::Escape) {
            get_char_pressed();
            return ApplicationState::Quit;
        }

        self.gui
            .draw_win(self.settings.word_length, game.get_guesses());

        ApplicationState::Game
    }

    fn run_loss_frame(&self, game: &mut Game) -> ApplicationState {
        if is_key_pressed(KeyCode::M) {
            get_char_pressed();
            return ApplicationState::Menu;
        }

        if is_key_pressed(KeyCode::N) {
            get_char_pressed();
            return ApplicationState::NewGame;
        }

        if is_key_pressed(KeyCode::Escape) {
            return ApplicationState::Quit;
        }

        self.gui.draw_loss(
            self.settings.word_length,
            game.get_guesses(),
            &game.get_correct_word(),
        );

        ApplicationState::Game
    }
}
