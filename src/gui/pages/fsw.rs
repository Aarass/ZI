use iced::{
    alignment,
    widget::{
        button, column, container, horizontal_space, row, text, text_input, toggler, vertical_space,
    },
    Element, Length,
};

use crate::algorithms::Operation;
use crate::gui::state::messages::{FSWPageMessage, Message};
use crate::State;

pub fn fsw_page(state: &State) -> Element<Message> {
    let can_run = state.fsw.from.is_some() && state.fsw.to.is_some();
    let from = state
        .fsw
        .from
        .as_ref()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .unwrap_or(String::from(""));
    let to = state
        .fsw
        .to
        .as_ref()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .unwrap_or(String::from(""));
    column![
        text("Directory which the file watcher will monitor"),
        row![
            text_input("Click the \"Choose\" button", &from)
                .width(Length::Fill)
                .on_input(|_| Message::Empty),
            button(text("Choose").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !state.fsw.is_on {
                    Some(Message::FSW(FSWPageMessage::GetDirToWatch))
                } else {
                    None
                })
        ],
        horizontal_space().height(10),
        text("Directory where the result will be saved"),
        row![
            text_input("Click the \"Choose\" button", &to)
                .width(Length::Fill)
                .on_input(|_| Message::Empty),
            button(text("Choose").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !state.fsw.is_on {
                    Some(Message::FSW(FSWPageMessage::GetDirToSaveTo))
                } else {
                    None
                })
        ],
        vertical_space().height(10),
        container(row![
            text("Decryption"),
            horizontal_space().width(10),
            toggler(matches!(state.fsw.mode, Operation::Encrypt))
                .spacing(0)
                .size(20)
                .on_toggle_maybe(if state.fsw.is_on {
                    None
                } else {
                    Some(|_| Message::FSW(FSWPageMessage::ToggleMode))
                }),
            horizontal_space().width(10),
            text("Encryption"),
        ])
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center),
        vertical_space().height(10),
        container(
            button(
                text(if state.fsw.is_on {
                    "Turn off"
                } else {
                    "Turn on"
                })
                .align_x(alignment::Horizontal::Center)
            )
            .width(Length::Shrink)
            .on_press_maybe(if can_run {
                Some(if state.fsw.is_on {
                    Message::FSW(FSWPageMessage::TurnOff)
                } else {
                    Message::FSW(FSWPageMessage::TurnOn)
                })
            } else {
                None
            })
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
    ]
    .padding([50, 100])
    .into()
}
