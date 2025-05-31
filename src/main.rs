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
        // .theme(|_| iced::Theme::SolarizedDark)
        // .theme(|_| iced::Theme::Nord)
        .theme(|_| iced::Theme::TokyoNightStorm)
        .window(window::Settings {
            icon,
            ..Default::default()
        })
        .window_size(Size::new(600.0, 400.0))
        .centered()
        .subscription(State::subscription);

    app.run()
}

// use algorithms::enigma::alg::Enigma;
// use algorithms::xxtea::alg::{Xxtea, XxteaCfb};
// use anyhow::anyhow;
// use gui::pages::fsw::fsw_page;
// use gui::pages::manual::manual_page;
// use gui::pages::settings::settings_page;
// use gui::pages::tcp::tcp_page;
// use iced::futures::sink::SinkExt;
// use iced::stream;
// use iced::{
//     alignment,
//     widget::{
//         button, column, container, horizontal_rule, horizontal_space, opaque, pick_list, row,
//         scrollable, stack, svg, text, text_input, toggler, vertical_space, Column, Row,
//     },
//     Alignment, Background, Border, Color, Element, Length, Shadow, Size, Subscription, Task, Theme,
// };
// use notify::{recommended_watcher, RecursiveMode, Watcher};
// use rfd::AsyncFileDialog;
// use std::net::Ipv4Addr;
// use std::ops::Deref;
// use std::path::Path;
// use std::str::FromStr;
// use std::time::{Duration, SystemTime};
// use std::{
//     fmt::Display,
//     path::PathBuf,
//     sync::{Arc, RwLock},
// };
// use tokio::fs;
// use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
// use tokio::{
//     net::{TcpListener, TcpStream},
//     task::JoinHandle,
// };
//
