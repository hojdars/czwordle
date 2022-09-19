use macroquad::prelude::*;

fn draw_word(x: f32, y: f32, word: &String, tp: &TextParams) {
    for (i, c) in (0_usize..word.len()).zip(word.chars()) {
        draw_text_ex(&c.to_string(), x + i as f32 * 35.0, y, *tp);
    }
}

fn draw_boxes(x: f32, y: f32, number: u32) {
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
        clear_background(BLACK);

        while let Some(c) = get_char_pressed() {
            match c {
                '\u{08}' => {
                    word.pop();
                    get_char_pressed();
                }
                '\u{0d}' => {
                    if word.chars().count() == LENGTH as usize {
                        words.push(word.to_uppercase());
                        word.clear();
                    }
                    break;
                }
                '\u{00}'..='\u{1F}' => {
                    continue;
                }
                _ => {
                    if word.chars().count() < LENGTH as usize{
                        for char in c.to_uppercase() {
                            word.push(char);
                        }
                    }
                }
            }
        }
        draw_word(120.0, 100.0, &word, &font_params);
        draw_boxes(113.0, 55.0, 5);

        for (i, w) in (1_usize..).zip(&words) {
            let posy = 100.0 + i as f32 * 80.0;
            draw_word(120.0, posy, &w.to_uppercase(), &font_params);
            draw_boxes(113.0, posy - 45.0, 5);
        }

        next_frame().await
    }
}