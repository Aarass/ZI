#![allow(clippy::needless_return)]

mod algorithms;
mod gui;
mod hash;
mod utils;

use gui::state::State;
use iced::{window, Size};

#[tokio::main]
async fn main() -> std::result::Result<(), iced::Error> {
    let icon = window::icon::from_file_data(include_bytes!("../assets/icon.png"), None).ok();

    let app = iced::application("ZI", State::update, State::view)
        .theme(|_| iced::Theme::SolarizedDark)
        .window(window::Settings {
            icon,
            ..Default::default()
        })
        .window_size(Size::new(600.0, 400.0))
        .centered()
        .subscription(State::subscription);

    app.run()
}

// Theme::TokyoNightStorm
// Theme::SolarizedDark
// Theme::Nord
