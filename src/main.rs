#![allow(unused)]
use std::path::Path;

use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use iced::{
    Element, Fill, Length, Point, Size, Theme,
    border::Radius,
    widget::{Container, button, center, column, container, row, text, text_input},
    window::{self, Id, Position::SpecificWith},
};

#[derive(Clone, Debug)]
enum Action {
    Forward,
    Backward,
    PlayPause,
}

#[derive(Default)]
struct State {
    media_state: String,
}

fn view(state: &State) -> Container<Action> {
    let mut media_state: &str;
    if state.media_state.as_str().is_empty() {
        media_state = "||";
    } else {
        media_state = state.media_state.as_str();
    }

    container(row![
        button("<<")
            .on_press(Action::Backward)
            .width(40)
            .style(|theme, style| button_with_base(theme, style, button::primary)),
        button(text(media_state).center())
            .on_press(Action::PlayPause)
            .width(40)
            .style(|theme, style| button_with_base(theme, style, button::primary)),
        button(">>")
            .on_press(Action::Forward)
            .width(40)
            .style(|theme, style| button_with_base(theme, style, button::primary)),
    ])
}

fn button_with_base(
    theme: &Theme,
    status: button::Status,
    f: fn(&Theme, button::Status) -> button::Style,
) -> button::Style {
    let mut style = f(theme, status);
    style.border.radius = Radius::default();
    style
}

fn update(state: &mut State, action: Action) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    match action {
        Action::Forward => {
            match enigo.key(Key::MediaNextTrack, Press) {
                Err(e) => {
                    eprintln!("Error while trying to skip track: {e}");
                }
                Ok(_) => {}
            };
            match enigo.key(Key::MediaNextTrack, Release) {
                Err(e) => {
                    eprintln!("Error while trying to skip track: {e}");
                }
                Ok(_) => {}
            };
        }
        Action::Backward => {
            match enigo.key(Key::MediaPrevTrack, Press) {
                Err(e) => {
                    eprintln!("Error while trying to move back a track: {e}");
                }
                Ok(_) => {}
            };
            match enigo.key(Key::MediaPrevTrack, Release) {
                Err(e) => {
                    eprintln!("Error while trying to move back a track: {e}");
                }
                Ok(_) => {}
            };
        }
        Action::PlayPause => {
            match enigo.key(Key::MediaPlayPause, Press) {
                Err(e) => {
                    eprintln!("Error while trying to pause / play track: {e}");
                }
                Ok(_) => {}
            };
            match enigo.key(Key::MediaPlayPause, Release) {
                Err(e) => {
                    eprintln!("Error while trying to pause / play track: {e}");
                }
                Ok(_) => {}
            };
            if state.media_state == "||" {
                state.media_state = "|>".to_string();
            } else {
                state.media_state = "||".to_string();
            }
        } // _ => {}
    }
}

fn minus(window: Size, screen: Size) -> Point {
    Point::new(screen.width - window.width, screen.height - window.height)
}

fn theme(state: &State) -> Theme {
    let white = iced::Color {
        r: (0.0),
        g: (0.0),
        b: (0.0),
        a: (1.0),
    };

    let black = iced::Color {
        r: (1.0),
        g: (1.0),
        b: (1.0),
        a: (1.0),
    };
    let red = iced::Color {
        r: (1.0),
        g: (0.0),
        b: (0.0),
        a: (1.0),
    };
    Theme::custom(
        "Custom Dark".to_string(),
        iced::theme::Palette {
            background: (black),
            text: (white),
            primary: (white),
            success: (white),
            danger: (red),
        },
    )
}

fn main() {
    iced::application("Indium", update, view)
        .window_size(Size::new(120.0, 30.0))
        .position(SpecificWith(minus))
        .level(iced::window::Level::AlwaysOnTop)
        .theme(theme)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .run()
        .expect("Error occured while running application");
}
