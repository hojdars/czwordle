use app::Settings;
use macroquad::prelude::*;

mod app;
use app::App;
use app::ApplicationState;
use app::MainMenuData;

mod gui;
use gui::menu::Menu;

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

async fn run_menu(application_state: &mut ApplicationState, app: &mut App<'_>) {
    let callback = |position: &mut u32, data: &mut MainMenuData, items: &Vec<String>| {
        if is_key_pressed(KeyCode::Enter) {
            match *position {
                0 => data.state = ApplicationState::NewGame,
                1 => data.state = ApplicationState::Quit,
                2 => println!("You got JEBAITED."),
                3 => println!("You got JEBAITED."),
                _ => panic!("cannot happen"),
            }
        } else if is_key_pressed(KeyCode::Escape) {
            data.state = ApplicationState::Quit;
        }

        let mut result_text: Vec<String> = Vec::new();
        for i in 0..items.len() {
            match i {
                0 => result_text.push(items[0].to_string()),
                1 => result_text.push(format!("{} {}", data.settings.attempts, items[1])),
                2 => result_text.push(format!("{} {}", data.settings.word_length, items[2])),
                3 => result_text.push(items[3].to_string()),
                _ => panic!("cannot happen"),
            }
        }
    };
    let mut main_menu = Menu::new(
        Vec::from([
            "NEW GAME".to_string(),
            "ATTEMPTS".to_string(),
            "WORD LENGTH".to_string(),
            "QUIT".to_string(),
        ]),
        MainMenuData {
            state: ApplicationState::Menu,
            settings: Settings {
                attempts: 6,
                word_length: 5,
            },
        },
        callback,
    );

    loop {
        *application_state = app.run_menu(&mut main_menu);

        match application_state {
            ApplicationState::Menu => {}
            ApplicationState::Quit => return,
            ApplicationState::Game => panic!("this should never happen"),
            ApplicationState::NewGame => break,
        }

        next_frame().await;
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
        std::include_str!("../data/jmena.txt"),
        load_fonts("ttf/NotoSansMono-Regular.ttf").await,
        load_texture("textures/logo.png").await.unwrap(),
        boxes,
    );

    let mut dictionary: Dictionary = app.make_dictionary();

    macroquad::rand::srand(instant::now() as u64);

    let mut application_state: ApplicationState = ApplicationState::Menu;

    loop {
        if application_state != ApplicationState::NewGame {
            run_menu(&mut application_state, &mut app).await;
            if application_state == ApplicationState::Quit {
                return;
            }
        }

        let mut game: Game = app.make_game(&mut dictionary);
        println!("{}", game.get_correct_word());

        loop {
            application_state = app.run_game(&mut game);
            match application_state {
                ApplicationState::Menu => break,
                ApplicationState::Quit => return,
                ApplicationState::Game => {}
                ApplicationState::NewGame => break,
            }

            next_frame().await
        }
    }
}
