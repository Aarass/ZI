use iced::{
    alignment,
    widget::{
        button, column, container, horizontal_space, row, text, text_input, toggler, vertical_space,
    },
    Element, Length,
};

use crate::{
    gui::state::{
        messages::{Message, TcpPageMessage},
        TcpMode,
    },
    utils::{valid_address, valid_port},
    State,
};

pub fn tcp_page(state: &State) -> Element<Message> {
    column![
        row![
            text("Send"),
            horizontal_space().width(10),
            toggler(!matches!(state.tcp.mode, TcpMode::Sending))
                .spacing(0)
                .size(20)
                .on_toggle_maybe(Some(|_| Message::Tcp(TcpPageMessage::ToggleMode))),
            horizontal_space().width(10),
            text("Receive")
        ]
        .align_y(alignment::Vertical::Center),
        vertical_space().height(Length::Fill),
        if let TcpMode::Sending = state.tcp.mode {
            tcp_send_widget(state)
        } else {
            tcp_recieve_widget(state)
        },
        vertical_space().height(Length::Fill),
    ]
    .align_x(alignment::Horizontal::Center)
    .padding([50, 100])
    .height(Length::Fill)
    .into()
}

fn tcp_send_widget(state: &State) -> Element<Message> {
    let file = state
        .tcp
        .file
        .as_ref()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .unwrap_or(String::from(""));
    let address = state.tcp.reciever_adress.as_deref().unwrap_or("");
    let port = state
        .tcp
        .reciever_port
        .map(|val| val.to_string())
        .unwrap_or(String::from(""));
    let is_sending = state.tcp.is_sending;
    let can_send = valid_address(&state.tcp.reciever_adress)
        && valid_port(&state.tcp.reciever_port)
        && state.tcp.file.is_some()
        && !is_sending;
    column![
        text("File to send"),
        row![
            text_input("Click the \"Choose\" button", &file)
                .width(Length::Fill)
                .on_input(|_| Message::Empty),
            button(text("Choose").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !is_sending {
                    Some(Message::Tcp(TcpPageMessage::SelectFileToSend))
                } else {
                    None
                }),
        ],
        vertical_space().height(10),
        row![
            text_input("Address", address)
                .on_input_maybe(if !is_sending {
                    Some(|value| Message::Tcp(TcpPageMessage::RecieverAddressChanged(value)))
                } else {
                    None
                })
                .width(Length::Fill),
            text(" : "),
            text_input("Port", &port)
                .on_input_maybe(if !is_sending {
                    Some(|value| Message::Tcp(TcpPageMessage::RecieverPortChanged(value)))
                } else {
                    None
                })
                .width(Length::Fill),
        ]
        .align_y(alignment::Vertical::Center),
        vertical_space().height(10),
        container(
            button(
                text(if state.tcp.is_sending {
                    "Sending..."
                } else {
                    "Send"
                })
                .align_x(alignment::Horizontal::Center)
            )
            .width(Length::Shrink)
            .on_press_maybe(if can_send {
                Some(Message::Tcp(TcpPageMessage::Send))
            } else {
                None
            })
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
    ]
    .into()
}

fn tcp_recieve_widget(state: &State) -> Element<Message> {
    let port = state
        .tcp
        .my_port
        .map(|val| val.to_string())
        .unwrap_or(String::from(""));
    let is_listening = state.tcp.is_listening;
    let to = state
        .tcp
        .dir_to_store_files
        .as_ref()
        .map(|path| path.clone().into_os_string().into_string().unwrap())
        .unwrap_or(String::from(""));
    let can_start_listening =
        valid_port(&state.tcp.my_port) && !is_listening && state.tcp.dir_to_store_files.is_some();
    column![
        text("Directory to save recieved files to"),
        row![
            text_input("Click the \"Choose\" button", &to)
                .width(Length::Fill)
                .on_input(|_| Message::Empty),
            button(text("Choose").align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !state.tcp.is_listening {
                    Some(Message::Tcp(TcpPageMessage::SelectDirToStoreFiles))
                } else {
                    None
                })
        ],
        vertical_space().height(10),
        row![
            text_input("127.0.0.1", "").width(Length::Fill),
            text(" : "),
            text_input("Port", &port)
                .on_input_maybe(if !is_listening {
                    Some(|value| Message::Tcp(TcpPageMessage::MyPortChanged(value)))
                } else {
                    None
                })
                .width(Length::Fill),
        ]
        .align_y(alignment::Vertical::Center),
        vertical_space().height(10),
        container(
            button(
                text(if is_listening {
                    "Stop listening"
                } else {
                    "Start Listening"
                })
                .align_x(alignment::Horizontal::Center)
            )
            .width(Length::Shrink)
            .on_press_maybe(if can_start_listening {
                Some(Message::Tcp(TcpPageMessage::StartListening))
            } else if is_listening {
                Some(Message::Tcp(TcpPageMessage::StopListening))
            } else {
                None
            })
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
    ]
    .into()
}
