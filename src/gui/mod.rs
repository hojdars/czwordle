use macroquad::prelude::*;

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

pub fn draw_words(current_word: &String, past_words: &Vec<String>, text_params: &TextParams) {
    for (i, w) in (0_usize..).zip(past_words) {
        let posy = 100.0 + i as f32 * 80.0;
        draw_word(120.0, posy, &w.to_uppercase(), text_params);
        draw_boxes(113.0, posy - 45.0, 5);
    }

    if past_words.len() >= 6 {
        return;
    }

    let posy = 100.0 + past_words.len() as f32 * 80.0;
    draw_word(120.0, posy, current_word, &text_params);
    draw_boxes(113.0, posy - 45.0, 5);
}