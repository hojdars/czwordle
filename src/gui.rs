use macroquad::prelude::*;

use crate::app::Settings;
use crate::game::Game;

use super::game::Guess;
use super::letters::Letters;

const BG_COLOR: Color = Color::new(0.92, 0.92, 0.91, 1.0);
const FG_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
const CORRECT_COLOR: Color = Color::new(0.11, 0.69, 0.13, 1.0);
const YELLOW_COLOR: Color = Color::new(0.93, 0.79, 0.16, 1.0);
const UNUSED_COLOR: Color = Color::new(0.83, 0.83, 0.83, 1.0);

pub struct Graphics {
    font: TextParams,
    logo: Texture2D,
    box_textures: Vec<Texture2D>,
}

impl Graphics {
    pub fn new(font: TextParams, logo: Texture2D, box_textures: Vec<Texture2D>) -> Graphics {
        Graphics {
            font: TextParams {
                color: FG_COLOR,
                ..font
            },
            logo,
            box_textures,
        }
    }

    pub fn draw_menu(&self, settings: &Settings) {
        macroquad::window::clear_background(BG_COLOR);

        let logo_y_start: f32 = screen_height() / 15.0;
        draw_texture(
            self.logo,
            screen_width() / 2. - self.logo.width() / 2.,
            logo_y_start,
            WHITE,
        );

        let menu_y_start = logo_y_start + self.logo.height() + 100.;

        self.draw_centered_text("[N] for new game", menu_y_start);
        self.draw_centered_text(
            format!("[↑↓] {} attempts", settings.attempts).as_str(),
            menu_y_start + 60.,
        );
        self.draw_centered_text(
            format!("[←→] {} word length", settings.word_length).as_str(),
            menu_y_start + 120.,
        );
        self.draw_centered_text("[Esc] to quit", menu_y_start + 180.);
    }

    pub fn draw_game(&self, settings: &Settings, game: &Game, word: &String) {
        macroquad::window::clear_background(BG_COLOR);

        self.draw_words(settings.word_length, word, game.get_guesses());
        self.draw_letters(game.get_letters(), settings.attempts);
    }

    pub fn draw_win(&self, word_length: u32, past_words: &Vec<Guess>) {
        macroquad::window::clear_background(BG_COLOR);

        let spacing: f32 = self.box_textures[0].width() * 1.1;
        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * spacing) / 2.0;
        for (i, guess) in (0_usize..).zip(past_words) {
            let posy = 60.0 + i as f32 * 60.0;
            self.draw_guess(guess, posx, posy);
        }
        let posy = 70.0 + (past_words.len() as f32 + 1.0) * 60.0;

        draw_text_ex(
            "[M] for menu",
            60.0,
            posy,
            TextParams {
                font_size: 34,
                color: FG_COLOR,
                ..self.font
            },
        );

        draw_text_ex(
            "[N] for new game",
            60.0,
            posy + 70.0,
            TextParams {
                font_size: 34,
                color: FG_COLOR,
                ..self.font
            },
        );
    }

    pub fn draw_loss(&self, word_length: u32, past_words: &Vec<Guess>, correct_word: &str) {
        macroquad::window::clear_background(BG_COLOR);

        let spacing: f32 = self.box_textures[0].width() * 1.1;
        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * spacing) / 2.0;
        for (i, guess) in (0_usize..).zip(past_words) {
            let posy = 60.0 + i as f32 * 60.0;
            self.draw_guess(guess, posx, posy);
        }

        let start_y = 90.0 + past_words.len() as f32 * 60.0;
        self.draw_lose_word(posx, start_y, &correct_word.to_string());

        draw_text_ex(
            "[M] for menu",
            60.0,
            start_y + 90.0,
            TextParams {
                font_size: 34,
                color: FG_COLOR,
                ..self.font
            },
        );

        draw_text_ex(
            "[N] for new game",
            60.0,
            start_y + 160.0,
            TextParams {
                font_size: 34,
                color: FG_COLOR,
                ..self.font
            },
        );
    }

    pub fn draw_letters(&self, letters: &Letters, total_guesses: u32) {
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
        let diacritic_rows = ["ěščřžýáíé", "ďťňóúů"];

        let start_y = (total_guesses + 1) as f32 * 70.0;

        let unused_params = TextParams {
            color: FG_COLOR,
            ..self.font
        };
        let used_params = TextParams {
            color: UNUSED_COLOR,
            ..self.font
        };
        let yellow_params = TextParams {
            color: YELLOW_COLOR,
            ..self.font
        };
        let green_params = TextParams {
            color: CORRECT_COLOR,
            ..self.font
        };

        for (row_number, letter_row) in (0usize..).zip(rows) {
            let start_x: f32 = macroquad::window::screen_width() / 2.0
                - (letter_row.chars().count() as f32 * 40.0) / 2.0;
            let pos_y: f32 = start_y + row_number as f32 * 55.0;

            for (i, l) in (0usize..).zip(
                letter_row
                    .chars()
                    .flat_map(|c| c.to_uppercase())
                    .collect::<String>()
                    .chars(),
            ) {
                let pos_x: f32 = start_x + i as f32 * 40.0;

                if letters.get_green_letters().contains(&l) {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, green_params);
                } else if letters.get_yellow_letters().contains(&l) {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, yellow_params);
                } else if letters.get_used_letters().contains(&l) {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, used_params);
                } else {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, unused_params);
                }
            }
        }

        for (row_number, letter_row) in (0usize..).zip(diacritic_rows) {
            let start_x: f32 = macroquad::window::screen_width() / 2.0
                - (letter_row.chars().count() as f32 * 40.0) / 2.0;
            let pos_y: f32 = start_y + (row_number + 4) as f32 * 55.0;

            for (i, l) in (0usize..).zip(
                letter_row
                    .chars()
                    .flat_map(|c| c.to_uppercase())
                    .collect::<String>()
                    .chars(),
            ) {
                let pos_x: f32 = start_x + i as f32 * 40.0;

                if letters.get_green_letters().contains(&l) {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, green_params);
                } else if letters.get_yellow_letters().contains(&l) {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, yellow_params);
                } else if letters.get_used_letters().contains(&l) {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, used_params);
                } else {
                    draw_text_ex(&l.to_string(), pos_x, pos_y, unused_params);
                }
            }
        }
    }

    fn draw_centered_text(&self, text: &str, pos_y: f32) {
        let dimensions: TextDimensions = measure_text(
            text,
            Some(self.font.font),
            self.font.font_size,
            self.font.font_scale,
        );

        let pos_x: f32 = screen_width() / 2.0 - dimensions.width / 2.0;
        draw_text_ex(
            text,
            pos_x,
            pos_y,
            TextParams {
                color: FG_COLOR,
                ..self.font
            },
        );
    }

    fn draw_letter(&self, letter: &str, pos_x: f32, pos_y: f32, texture_index: usize) {
        assert!(letter.chars().count() == 1);
        assert!(texture_index < self.box_textures.len());

        let c = get_text_center(
            letter,
            Some(self.font.font),
            self.font.font_size,
            self.font.font_scale,
            self.font.rotation,
        );

        let anchor_y = pos_y + self.box_textures[texture_index].height() * 0.35;
        draw_texture(
            self.box_textures[texture_index],
            pos_x - self.box_textures[texture_index].width() / 2.0,
            pos_y - self.box_textures[texture_index].height() / 2.0,
            WHITE,
        );
        draw_text_ex(letter, pos_x - c.x, anchor_y, self.font);
    }

    fn draw_words(&self, word_length: u32, current_word: &String, past_words: &Vec<Guess>) {
        let spacing: f32 = self.box_textures[0].width() * 1.1;
        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * spacing) / 2.0;

        for (i, guess) in (0_usize..).zip(past_words) {
            let posy = 60.0 + i as f32 * 60.0;
            self.draw_guess(guess, posx, posy);
        }

        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * spacing) / 2.0;
        let posy = 60.0 + past_words.len() as f32 * 60.0;
        self.draw_word(posx, posy, current_word);
    }

    fn draw_word(&self, x: f32, y: f32, word: &String) {
        let spacing: f32 = self.box_textures[0].width() * 1.1;
        for (i, c) in (0_usize..word.len()).zip(word.chars()) {
            self.draw_letter(&c.to_string(), x + i as f32 * spacing, y, 2);
        }
    }

    fn draw_lose_word(&self, x: f32, y: f32, word: &String) {
        let spacing: f32 = self.box_textures[0].width() * 1.1;
        for (i, c) in (0_usize..word.len()).zip(word.chars()) {
            self.draw_letter(&c.to_string(), x + i as f32 * spacing, y, 3);
        }
    }

    fn draw_guess(&self, guess: &Guess, x: f32, y: f32) {
        let spacing: f32 = self.box_textures[0].width() * 1.1;

        for (i, letter) in (0_u32..).zip(guess.word.chars()) {
            let mut texture_index: usize = 2;

            if guess.green_positions.contains(&i) {
                texture_index = 1;
            } else if guess.yellow_positions.contains(&i) {
                texture_index = 0;
            }

            self.draw_letter(
                letter.to_string().as_str(),
                x + i as f32 * spacing,
                y,
                texture_index,
            )
        }
    }
}
