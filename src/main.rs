use std::panic;

use macroquad::prelude::*;

mod app;
use app::App;
use app::ApplicationState;

mod gui;

mod dictionary;
use dictionary::Dictionary;

mod letters;

mod game;

async fn load_fonts(path: &str) -> TextParams {
    let pf = load_ttf_font(path).await;
    let poppins_font = pf.unwrap();

    TextParams {
        font_size: 42,
        font: poppins_font,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: Color::new(1.0, 1.0, 0.0, 1.0),
        rotation: 0.0,
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "czWORDLE".to_owned(),
        fullscreen: false,
        window_height: 800,
        window_width: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut boxes: Vec<Texture2D> = Vec::new();
    boxes.push(load_texture("textures/yellow.png").await.unwrap());
    boxes.push(load_texture("textures/green.png").await.unwrap());
    boxes.push(load_texture("textures/gray.png").await.unwrap());
    boxes.push(load_texture("textures/red.png").await.unwrap());

    let mut app = App::new(
        std::include_str!("../data/dictionary.txt"),
        load_fonts("ttf/NotoSansMono-Regular.ttf").await,
        load_texture("textures/logo.png").await.unwrap(),
        boxes,
    );

    let mut dictionary: Dictionary = app.make_dictionary();

    macroquad::rand::srand(instant::now() as u64);

    let mut application_state: ApplicationState = ApplicationState::Menu;

    loop {
        if application_state != ApplicationState::NewGame {
            let result: ApplicationState = app.run_menu_loop().await;
            match result {
                ApplicationState::Quit => return,
                ApplicationState::NewGame => {}
                _ => panic!("this should never happen"),
            }
        }

        application_state = app.run_game_loop(&mut dictionary).await;
        match application_state {
            ApplicationState::Quit => return,
            ApplicationState::Game => panic!("this should never happen"),
            _ => {}
        }
    }
}
