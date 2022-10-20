use macroquad::prelude::*;

use super::game::Guess;
use super::letters::Letters;

pub fn draw_word(x: f32, y: f32, word: &String, tp: &TextParams) {
    for (i, c) in (0_usize..word.len()).zip(word.chars()) {
        draw_text_ex(&c.to_string(), x + i as f32 * 35.0, y, *tp);
    }
}

pub fn draw_boxes(x: f32, y: f32, number: u32) {
    for i in 0_u32..number {
        draw_rectangle_lines(
            x + i as f32 * 35.0,
            y,
            36.0,
            60.0,
            2.0,
            Color {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 1.0,
            },
        );
    }
}

fn draw_guess(guess: &Guess, x: f32, y: f32, text_params: &TextParams) {
    for (i, letter) in (0_u32..).zip(guess.word.chars()) {
        let mut color = Color::new(1.0, 1.0, 1.0, 1.0);

        if guess.green_positions.contains(&i) {
            color = Color::new(0.0, 1.0, 0.0, 1.0);
        } else if guess.yellow_positions.contains(&i) {
            color = Color::new(1.0, 1.0, 0.0, 1.0);
        }

        let mut tp = *text_params;
        tp.color = color;
        draw_text_ex(letter.to_string().as_str(), x + i as f32 * 35.0, y, tp);
    }
}

pub fn draw_guesses(word_length: u32, past_words: &Vec<Guess>, text_params: &TextParams) {
    let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * 35.0) / 2.0;
    for (i, guess) in (0_usize..).zip(past_words) {
        let posy = 120.0 + i as f32 * 80.0;
        draw_guess(guess, posx, posy, text_params);
    }
}

fn draw_input_box(
    word_length: u32,
    guesses_len: usize,
    current_word: &String,
    text_params: &TextParams,
) {
    let posx = macroquad::window::screen_width() / 2.0 - (word_length as f32 * 35.0) / 2.0;
    let posy = 120.0 + guesses_len as f32 * 80.0;
    draw_word(posx, posy, current_word, text_params);
    draw_boxes(posx - 7.0, posy - 45.0, word_length);
}

pub fn draw_words(
    word_length: u32,
    current_word: &String,
    past_words: &Vec<Guess>,
    text_params: &TextParams,
) {
    draw_guesses(word_length, past_words, text_params);

    draw_input_box(word_length, past_words.len(), current_word, text_params);
}

pub fn draw_menu(tries: u32, length: u32, text_params: &TextParams) {
    let mut title: TextParams = *text_params;
    title.font_size = 60;
    title.color = Color::new(1.0, 0.4, 0.0, 1.0);

    draw_text_ex(
        "czWORDLE",
        macroquad::window::screen_width() / 2.0 - 110.0,
        100.0,
        title,
    );

    draw_text_ex("[N] for new game", 60.0, 180.0, *text_params);
    draw_text_ex(
        format!("[↑↓] {} attempts", tries).as_str(),
        60.0,
        240.0,
        *text_params,
    );
    draw_text_ex(
        format!("[←→] {} word length", length).as_str(),
        60.0,
        300.0,
        *text_params,
    );
    draw_text_ex("[Esc] to quit", 60.0, 360.0, *text_params);
}

pub fn draw_win(word_length: u32, past_words: &Vec<Guess>, text_params: &TextParams) {
    draw_guesses(word_length, past_words, text_params);

    let mut win: TextParams = *text_params;
    win.font_size = 34;
    win.color = Color::new(0.0, 1.0, 0.0, 1.0);

    let posy = 120.0 + (past_words.len() as f32 + 1.0) * 80.0;
    draw_text_ex("You win. Press M for menu.", 60.0, posy, win);
}

pub fn draw_letters(letters: &Letters, total_guesses: u32, text_params: &TextParams) {
    let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
    let diacritic_rows = ["ěščřžýáíé", "ďťňóúů"];

    let start_y = 160.0 + total_guesses as f32 * 80.0;

    let unused_params = TextParams {
        color: Color::new(1.0, 1.0, 1.0, 1.0),
        ..*text_params
    };
    let used_params = TextParams {
        color: Color::new(0.3, 0.3, 0.3, 1.0),
        ..*text_params
    };
    let yellow_params = TextParams {
        color: Color::new(1.0, 1.0, 0.0, 1.0),
        ..*text_params
    };
    let green_params = TextParams {
        color: Color::new(0.0, 1.0, 0.0, 1.0),
        ..*text_params
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

pub fn draw_loss(
    word_length: u32,
    past_words: &Vec<Guess>,
    correct_word: &str,
    text_params: &TextParams,
) {
    let red_text = TextParams {
        color: Color::new(1.0, 0.0, 0.0, 1.0),
        ..*text_params
    };

    draw_guesses(word_length, past_words, text_params);

    let start_y = 160.0 + past_words.len() as f32 * 80.0;

    draw_text_ex(correct_word, 130.0, start_y, red_text);
}
