use macroquad::prelude::*;
use std::cmp;

use crate::gui::graphics;
use crate::gui::graphics::Graphics;

pub struct Menu<'a, T: std::fmt::Debug + Copy> {
    items: Vec<String>,
    data: T,
    callback: Box<dyn FnMut(&mut u32, &mut T) + 'a>,
    items_callback: Option<Box<dyn FnMut(&mut T, &Vec<String>) -> Vec<String> + 'a>>,
    position: u32,
}

impl<'a, T: std::fmt::Debug + Copy> Menu<'a, T> {
    pub fn new(items: Vec<String>, data: T, cb: impl FnMut(&mut u32, &mut T) + 'a) -> Menu<'a, T> {
        Menu {
            items,
            data,
            callback: Box::new(cb),
            items_callback: None,
            position: 0,
        }
    }

    pub fn new_with_items_callback(
        items: Vec<String>,
        data: T,
        cb: impl FnMut(&mut u32, &mut T) + 'a,
        items_cb: impl FnMut(&mut T, &Vec<String>) -> Vec<String> + 'a,
    ) -> Menu<'a, T> {
        Menu {
            items,
            data,
            callback: Box::new(cb),
            items_callback: Some(Box::new(items_cb)),
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

        (self.callback)(&mut self.position, &mut self.data);

        if self.items_callback.is_some() {
            let modified_items: Vec<String> =
                self.items_callback.as_mut().unwrap()(&mut self.data, &self.items);
            self.draw(&modified_items, y_start, graphics);
        } else {
            self.draw(&self.items, y_start, graphics);
        }

        self.data
    }

    fn draw(&self, items: &Vec<String>, y_start: f32, graphics: &mut Graphics) {
        let mut num: u32 = 0;
        for item in items {
            let mut color: Color = graphics::FG_COLOR;

            if self.position == num {
                color = graphics::CORRECT_COLOR;
            }

            graphics.draw_centered_text(item.as_str(), y_start + 60.0 * num as f32, color);
            num += 1;
        }
    }
}
