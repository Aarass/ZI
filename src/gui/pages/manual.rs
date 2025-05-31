use iced::{
    alignment,
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Element, Length,
};

use crate::gui::state::messages::{ManualPageMessage, Message};
use crate::State;

pub fn manual_page(state: &State) -> Element<Message> {
    let can_run =
        state.manual.from.is_some() && state.manual.to.is_some() && !state.manual.is_doing_work;
    let from = state
        .manual
        .from
        .as_ref()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .unwrap_or(String::from(""));
    let to = state
        .manual
        .to
        .as_ref()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .unwrap_or(String::from(""));
    column![
        text("File to process"),
        row![
            text_input("Click the \"Choose\" button", &from)
                .width(Length::Fill)
                .on_input(|_| Message::Empty),
            button(text("Choose").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !state.manual.is_doing_work {
                    Some(Message::Manual(ManualPageMessage::GetFile))
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
                .on_press_maybe(if !state.manual.is_doing_work {
                    Some(Message::Manual(ManualPageMessage::GetDirToSaveTo))
                } else {
                    None
                })
        ],
        horizontal_space().height(10),
        container(row![
            button(text("Encrypt").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if can_run {
                    Some(Message::Manual(ManualPageMessage::StartEncryption))
                } else {
                    None
                }),
            horizontal_space().width(10),
            button(text("Decrypt").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if can_run {
                    Some(Message::Manual(ManualPageMessage::StartDecryption))
                } else {
                    None
                })
        ])
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
    ]
    .padding([50, 100])
    .into()
}
