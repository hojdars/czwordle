use macroquad::prelude::*;
use std::cmp;

use crate::gui::graphics;
use crate::gui::graphics::Graphics;

type InputCallback<'a, T> = Box<dyn FnMut(&mut u32, &mut T) + 'a>;
type ItemsCallback<'a, T> = Option<Box<dyn FnMut(&mut T, &Vec<String>) -> Vec<String> + 'a>>;

pub struct Menu<'a, T: std::fmt::Debug + Copy> {
    items: Vec<String>,
    data: T,
    callback: InputCallback<'a, T>,
    items_callback: ItemsCallback<'a, T>,
    position: u32,
}

impl<'a, T: std::fmt::Debug + Copy> Menu<'a, T> {
    pub fn new(
        items: Vec<String>,
        data: T,
        callback: impl FnMut(&mut u32, &mut T) + 'a,
    ) -> Menu<'a, T> {
        Menu {
            items,
            data,
            callback: Box::new(callback),
            items_callback: None,
            position: 0,
        }
    }

    pub fn new_with_items_callback(
        items: Vec<String>,
        data: T,
        callback: impl FnMut(&mut u32, &mut T) + 'a,
        items_callback: impl FnMut(&mut T, &Vec<String>) -> Vec<String> + 'a,
    ) -> Menu<'a, T> {
        Menu {
            items,
            data,
            callback: Box::new(callback),
            items_callback: Some(Box::new(items_callback)),
            position: 0,
        }
    }

    pub fn run(&mut self, y_start: f32, graphics: &mut Graphics) -> T {
        if is_key_pressed(KeyCode::Down) {
            if self.position == (self.items.len() - 1) as u32 {
                self.position = 0;
            } else {
                self.position = cmp::min(self.position + 1, (self.items.len() - 1) as u32);
            }
        } else if is_key_pressed(KeyCode::Up) {
            if self.position > 0 {
                self.position = cmp::max(0, self.position - 1);
            } else {
                self.position = (self.items.len() - 1) as u32;
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

    fn draw(&self, items: &[String], y_start: f32, graphics: &mut Graphics) {
        for (num, item) in (0_u32..).zip(items.iter()) {
            let mut color: Color = graphics::FG_COLOR;

            if self.position == num {
                color = graphics::CORRECT_COLOR;
            }

            graphics.draw_centered_text(item.as_str(), y_start + 60.0 * num as f32, color);
        }
    }
}
