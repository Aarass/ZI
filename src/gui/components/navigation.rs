use std::path::PathBuf;

use iced::{
    alignment,
    widget::{button, horizontal_space, row, svg, text},
    Border, Color, Element, Length, Shadow, Theme,
};

use crate::gui::{
    pages::Page,
    state::{
        messages::{Message, NavigationMessage},
        State,
    },
};

const TAB_RADIUS: f32 = 10.0;

pub fn navigation(state: &State) -> Element<Message> {
    row![
        button(text("FS Watcher").align_x(alignment::Horizontal::Center))
            .width(150)
            .on_press(Message::Navigation(NavigationMessage::GoToFSWPage))
            .style(move |theme: &Theme, status| {
                let mut style = if let Page::Fsw = state.page {
                    button::primary(theme, status)
                } else {
                    button::secondary(theme, status)
                };

                style.border.radius = iced::border::Radius {
                    top_left: TAB_RADIUS,
                    top_right: TAB_RADIUS,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                };

                style
            }),
        button(text("Manual").align_x(alignment::Horizontal::Center))
            .width(150)
            .on_press(Message::Navigation(NavigationMessage::GoToManualPage))
            .style(move |theme: &Theme, status| {
                let mut style = if let Page::Manual = state.page {
                    button::primary(theme, status)
                } else {
                    button::secondary(theme, status)
                };

                style.border.radius = iced::border::Radius {
                    top_left: TAB_RADIUS,
                    top_right: TAB_RADIUS,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                };

                style
            }),
        button(text("Tcp").align_x(alignment::Horizontal::Center))
            .width(150)
            .on_press(Message::Navigation(NavigationMessage::GoToTcpPage))
            .style(move |theme: &Theme, status| {
                let mut style = if let Page::Tcp = state.page {
                    button::primary(theme, status)
                } else {
                    button::secondary(theme, status)
                };

                style.border.radius = iced::border::Radius {
                    top_left: TAB_RADIUS,
                    top_right: TAB_RADIUS,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                };

                style
            }),
        horizontal_space(),
        button(row![
            text(""),
            svg(svg::Handle::from_path(PathBuf::from("./assets/gear.svg")))
                .width(20)
                .height(20)
        ])
        .width(Length::Shrink)
        .on_press(Message::Navigation(NavigationMessage::GoToSettingsPage))
        .style(|_, _| {
            button::Style {
                background: None,
                text_color: Color::BLACK,
                border: Border::default().rounded(500),
                shadow: Shadow::default(),
            }
        })
        .style(move |theme: &Theme, status| {
            let mut style = if let Page::Settings = state.page {
                button::primary(theme, status)
            } else {
                button::secondary(theme, status)
            };

            style.border.radius = iced::border::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: TAB_RADIUS,
            };

            style
        }),
    ]
    .spacing(1)
    .into()
}
