use macroquad::prelude::*;

mod gui;
use gui::draw_words;

#[macroquad::main("CZWORDLE")]
async fn main() {
    let mut word = String::new();

    let pf = load_ttf_font("ttf/UbuntuMono-Regular.ttf").await;
    let poppins_font = pf.unwrap();

    let font_params = TextParams {
        font_size: 42,
        font: poppins_font,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: Color::new(1.0, 1.0, 0.0, 1.0),
    };

    const TRIES: u32 = 6;
    const LENGTH: u32 = 5;

    let mut words: Vec<String> = vec![];

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Enter) {
            if word.chars().count() == LENGTH as usize && words.len() < TRIES as usize {
                words.push(word.to_uppercase());
                word.clear();
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            word.pop();
            get_char_pressed();
        }

        clear_background(BLACK);

        while let Some(c) = get_char_pressed() {
            match c {
                '\u{00}'..='\u{1F}' => {
                    continue;
                }
                _ => {
                    if word.chars().count() < LENGTH as usize {
                        for char in c.to_uppercase() {
                            word.push(char);
                        }
                    }
                }
            }
        }

        draw_words(&word, &words, &font_params);

        next_frame().await
    }
}
