use macroquad::prelude::*;

use crate::app::Settings;
use crate::game::Game;

use super::game::Guess;
use super::letters::Letters;

pub struct Graphics {
    font: TextParams,
}

const BG_COLOR: Color = Color::new(0.1, 0.1, 0.1, 1.0);
const FG_COLOR: Color = Color::new(1.0, 1.0, 0.0, 1.0);

impl Graphics {
    pub fn new(font: TextParams) -> Graphics {
        Graphics {
            font: TextParams {
                color: FG_COLOR,
                ..font
            },
        }
    }

    pub fn draw_menu(&self, settings: &Settings) {
        macroquad::window::clear_background(BG_COLOR);

        draw_text_ex(
            "czWORDLE",
            macroquad::window::screen_width() / 2.0 - 110.0,
            100.0,
            TextParams {
                font_size: 60,
                ..self.font
            },
        );

        draw_text_ex("[N] for new game", 60.0, 180.0, self.font);
        draw_text_ex(
            format!("[↑↓] {} attempts", settings.attempts).as_str(),
            60.0,
            240.0,
            self.font,
        );
        draw_text_ex(
            format!("[←→] {} word length", settings.word_length).as_str(),
            60.0,
            300.0,
            self.font,
        );
        draw_text_ex("[Esc] to quit", 60.0, 360.0, self.font);
    }

    pub fn draw_game(&self, settings: &Settings, game: &Game, word: &String) {
        macroquad::window::clear_background(BG_COLOR);

        self.draw_words(
            settings.word_length,
            word,
            game.get_guesses(),
            settings.attempts,
        );
        self.draw_letters(game.get_letters(), settings.attempts);
    }

    pub fn draw_win(&self, word_length: u32, past_words: &Vec<Guess>) {
        macroquad::window::clear_background(BG_COLOR);

        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * 35.0) / 2.0;
        for (i, guess) in (0_usize..).zip(past_words) {
            let posy = 120.0 + i as f32 * 80.0;
            self.draw_guess(guess, posx, posy);
        }
        let posy = 120.0 + (past_words.len() as f32 + 1.0) * 80.0;
        draw_text_ex(
            "You win. Press M for menu.",
            60.0,
            posy,
            TextParams {
                font_size: 34,
                color: Color::new(0.0, 1.0, 0.0, 1.0),
                ..self.font
            },
        );
    }

    pub fn draw_loss(&self, word_length: u32, past_words: &Vec<Guess>, correct_word: &str) {
        macroquad::window::clear_background(BG_COLOR);

        let red_text = TextParams {
            color: Color::new(1.0, 0.0, 0.0, 1.0),
            ..self.font
        };

        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * 35.0) / 2.0;
        for (i, guess) in (0_usize..).zip(past_words) {
            let posy = 120.0 + i as f32 * 80.0;
            self.draw_guess(guess, posx, posy);
        }
        let start_y = 160.0 + past_words.len() as f32 * 80.0;

        draw_text_ex(correct_word, 130.0, start_y, red_text);
    }

    fn draw_words(
        &self,
        word_length: u32,
        current_word: &String,
        past_words: &Vec<Guess>,
        attempts: u32,
    ) {
        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * 35.0) / 2.0;

        for i in 0..attempts {
            let posy = 60.0 + i as f32 * 70.0;
            self.draw_boxes(posx - 7.0, posy - 45.0, word_length);
        }

        for (i, guess) in (0_usize..).zip(past_words) {
            let posy = 60.0 + i as f32 * 70.0;
            self.draw_guess(guess, posx, posy);
        }

        let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * 35.0) / 2.0;
        let posy = 60.0 + past_words.len() as f32 * 70.0;
        self.draw_boxes(posx - 7.0, posy - 45.0, word_length);
        self.draw_word(posx, posy, current_word);
    }

    fn draw_word(&self, x: f32, y: f32, word: &String) {
        for (i, c) in (0_usize..word.len()).zip(word.chars()) {
            draw_text_ex(&c.to_string(), x + i as f32 * 35.0, y, self.font);
        }
    }

    fn draw_boxes(&self, x: f32, y: f32, number: u32) {
        for i in 0_u32..number {
            draw_rectangle(
                x + i as f32 * 35.0,
                y,
                32.0,
                60.0,
                Color {
                    r: 0.15,
                    g: 0.15,
                    b: 0.15,
                    a: 1.0,
                },
            );
        }
    }

    fn draw_guess(&self, guess: &Guess, x: f32, y: f32) {
        for (i, letter) in (0_u32..).zip(guess.word.chars()) {
            let mut color = Color::new(1.0, 1.0, 1.0, 1.0);

            if guess.green_positions.contains(&i) {
                color = Color::new(0.0, 1.0, 0.0, 1.0);
            } else if guess.yellow_positions.contains(&i) {
                color = Color::new(1.0, 1.0, 0.0, 1.0);
            }

            draw_text_ex(
                letter.to_string().as_str(),
                x + i as f32 * 35.0,
                y,
                TextParams { color, ..self.font },
            );
        }
    }

    pub fn draw_letters(&self, letters: &Letters, total_guesses: u32) {
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
        let diacritic_rows = ["ěščřžýáíé", "ďťňóúů"];

        let start_y = (total_guesses + 1) as f32 * 70.0;

        let unused_params = TextParams {
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..self.font
        };
        let used_params = TextParams {
            color: Color::new(0.3, 0.3, 0.3, 1.0),
            ..self.font
        };
        let yellow_params = TextParams {
            color: Color::new(1.0, 1.0, 0.0, 1.0),
            ..self.font
        };
        let green_params = TextParams {
            color: Color::new(0.0, 1.0, 0.0, 1.0),
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
}
