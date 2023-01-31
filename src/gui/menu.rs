use macroquad::prelude::*;
use std::cmp;

use crate::gui::graphics;
use crate::gui::graphics::Graphics;

pub struct Menu<'a, T: std::fmt::Debug + Copy> {
    items: Vec<String>,
    data: T,
    callback: Box<dyn FnMut(&mut u32, &mut T, &Vec<String>) + 'a>,
    position: u32,
}

impl<'a, T: std::fmt::Debug + Copy> Menu<'a, T> {
    pub fn new(
        items: Vec<String>,
        data: T,
        cb: impl FnMut(&mut u32, &mut T, &Vec<String>) + 'a,
    ) -> Menu<'a, T> {
        Menu {
            items,
            data,
            callback: Box::new(cb),
            position: 0,
        }
    }

    pub fn run(&mut self, y_start: f32, graphics: &mut Graphics) -> T {
        if is_key_pressed(KeyCode::Down) {
            self.position = cmp::min(self.position + 1, (self.items.len() - 1) as u32);
        } else if is_key_pressed(KeyCode::Up) {
            if self.position > 0 {
                self.position = cmp::max(0, self.position - 1);
            }
        }

        let to_draw: Vec<String> = (self.callback)(&mut self.position, &mut self.data, &self.items);

        self.draw(y_start, graphics);

        self.data
    }

    fn draw(&self, y_start: f32, graphics: &mut Graphics) {
        let mut num: u32 = 0;
        for item in &self.items {
            let mut color: Color = graphics::FG_COLOR;

            if self.position == num {
                color = graphics::CORRECT_COLOR;
            }

            graphics.draw_centered_text(item.as_str(), y_start + 60.0 * num as f32, color);
            num += 1;
        }
    }
}
