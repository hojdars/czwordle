use macroquad::prelude::*;

mod app;
use app::App;
use app::ApplicationState;

mod gui;

mod dictionary;
use dictionary::Dictionary;

mod letters;

mod game;
use game::Game;

async fn load_fonts(path: &str) -> TextParams {
    let pf = load_ttf_font(path).await;
    let poppins_font = pf.unwrap();

    TextParams {
        font_size: 42,
        font: poppins_font,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: Color::new(1.0, 1.0, 0.0, 1.0),
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "czWORDLE".to_owned(),
        fullscreen: false,
        window_height: 1000,
        window_width: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new(
        std::include_str!("../data/jmena.txt"),
        load_fonts("ttf/NotoSansMono-Regular.ttf").await,
    );

    let mut dictionary: Dictionary = app.make_dictionary();

    loop {
        let mut application_state: ApplicationState;

        loop {
            application_state = app.run_menu();
            match application_state {
                ApplicationState::Menu => {}
                ApplicationState::Quit => return,
                ApplicationState::Game => panic!("this should never happen"),
                ApplicationState::NewGame => break,
            }

            next_frame().await;
        }

        {
            let mut game: Game = app.make_game(&mut dictionary);
            loop {
                application_state = app.run_game(&mut game);
                match application_state {
                    ApplicationState::Menu => break,
                    ApplicationState::Quit => return,
                    ApplicationState::Game => {}
                    ApplicationState::NewGame => panic!("this should never happen"),
                }

                next_frame().await
            }
        }
    }
}
