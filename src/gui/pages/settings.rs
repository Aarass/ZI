use iced::{
    alignment,
    widget::{button, column, container, pick_list, row, text, text_input, vertical_space},
    Alignment, Element, Length,
};

use crate::{
    algorithms::AlgorithmOption,
    gui::state::{
        args::{EnigmaArgs, XxteaArgs, XxteaCfbArgs},
        messages::{
            AlgorithmSettingsMessage, EnigmaSettingsMessage, Message, XxteaCfbSettingsMessage,
            XxteaSettingsMessage,
        },
    },
    State,
};

pub fn settings_page(state: &State) -> Element<Message> {
    let option = state.settings.algorithm_option;

    let args: Element<Message> = match option {
        AlgorithmOption::Enigma => enigma_settings(&state.settings.enigma_args),
        AlgorithmOption::Xxtea => xxtea_settings(&state.settings.xxtea_args),
        AlgorithmOption::XxteaCfb => xxtea_cfb_settings(&state.settings.xxtea_cfb_args),
    };

    column![
        vertical_space().height(30),
        row![
            text("Algorithm: "),
            pick_list(
                vec![
                    AlgorithmOption::Enigma,
                    AlgorithmOption::Xxtea,
                    AlgorithmOption::XxteaCfb
                ],
                Some(option),
                Message::AlgorithmChanged
            ),
        ]
        .align_y(Alignment::Center),
        container(column![args])
            .center_y(Length::Fill)
            .padding([0, 50]),
        button(text("Save").align_x(alignment::Horizontal::Center))
            .width(Length::Shrink)
            .on_press(Message::CommitSettings),
        vertical_space().height(30),
    ]
    .height(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .into()
}

fn enigma_settings(state: &EnigmaArgs) -> Element<Message> {
    column![
        row![
            column![
                text("Reflector")
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                text_input("Wiring", state.refl_wiring.as_deref().unwrap_or(""))
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::ReflWiringChanged(value),
                        ))
                    })
                    .width(Length::Fill)
            ]
            .spacing(5),
            column![
                text("Rotor 1")
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                text_input("Wiring", state.rot1_wiring.as_deref().unwrap_or(""))
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::Rot1WiringChanged(value),
                        ))
                    })
                    .width(Length::Fill),
                row![
                    text_input("Notch", state.rot1_notch.as_deref().unwrap_or("")).on_input(
                        |val| {
                            let value = if val.is_empty() { None } else { Some(val) };
                            Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                                EnigmaSettingsMessage::Rot1NotchChanged(value),
                            ))
                        }
                    ),
                    text_input(
                        "Ringstellung",
                        state.rot1_ringstellung.as_deref().unwrap_or("")
                    )
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::Rot1RingstellungChanged(value),
                        ))
                    }),
                    text_input("Start", state.rot1_position.as_deref().unwrap_or("")).on_input(
                        |val| {
                            let value = if val.is_empty() { None } else { Some(val) };
                            Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                                EnigmaSettingsMessage::Rot1PositionChanged(value),
                            ))
                        }
                    ),
                ]
                .spacing(5)
            ]
            .spacing(5),
            column![
                text("Rotor 2")
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                text_input("Wiring", state.rot2_wiring.as_deref().unwrap_or(""))
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::Rot2WiringChanged(value),
                        ))
                    })
                    .width(Length::Fill),
                row![
                    text_input("Notch", state.rot2_notch.as_deref().unwrap_or("")).on_input(
                        |val| {
                            let value = if val.is_empty() { None } else { Some(val) };
                            Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                                EnigmaSettingsMessage::Rot2NotchChanged(value),
                            ))
                        }
                    ),
                    text_input(
                        "Ringstellung",
                        state.rot2_ringstellung.as_deref().unwrap_or("")
                    )
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::Rot2RingstellungChanged(value),
                        ))
                    }),
                    text_input("Start", state.rot2_position.as_deref().unwrap_or("")).on_input(
                        |val| {
                            let value = if val.is_empty() { None } else { Some(val) };
                            Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                                EnigmaSettingsMessage::Rot2PositionChanged(value),
                            ))
                        }
                    ),
                ]
                .spacing(5)
            ]
            .spacing(5),
            column![
                text("Rotor 3")
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                text_input("Wiring", state.rot3_wiring.as_deref().unwrap_or(""))
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::Rot3WiringChanged(value),
                        ))
                    })
                    .width(Length::Fill),
                row![
                    text_input("Notch", state.rot3_notch.as_deref().unwrap_or("")).on_input(
                        |val| {
                            let value = if val.is_empty() { None } else { Some(val) };
                            Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                                EnigmaSettingsMessage::Rot3NotchChanged(value),
                            ))
                        }
                    ),
                    text_input(
                        "Ringstellung",
                        state.rot3_ringstellung.as_deref().unwrap_or("")
                    )
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                            EnigmaSettingsMessage::Rot3RingstellungChanged(value),
                        ))
                    }),
                    text_input("Start", state.rot3_position.as_deref().unwrap_or("")).on_input(
                        |val| {
                            let value = if val.is_empty() { None } else { Some(val) };
                            Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                                EnigmaSettingsMessage::Rot3PositionChanged(value),
                            ))
                        }
                    ),
                ]
                .spacing(5)
            ]
            .spacing(5),
        ]
        .spacing(10),
        vertical_space().height(30),
        column![
            text("Plugboard")
                .width(Length::Fill)
                .align_x(Alignment::Center),
            text_input("Pairs", state.plugboard.as_deref().unwrap_or(""))
                .width(Length::Fill)
                .on_input(|val| {
                    let value = if val.is_empty() { None } else { Some(val) };
                    Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Enigma(
                        EnigmaSettingsMessage::PlugboardChanged(value),
                    ))
                }),
        ]
        .width(300)
        .spacing(5)
    ]
    .align_x(Alignment::Center)
    .into()
}

fn xxtea_settings(state: &XxteaArgs) -> Element<Message> {
    column![
        text("Key").width(Length::Fill),
        text_input("Key", state.key.as_deref().unwrap_or(""))
            .on_input(|val| {
                let value = if val.is_empty() { None } else { Some(val) };
                Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::Xxtea(
                    XxteaSettingsMessage::KeyChanged(value),
                ))
            })
            .width(Length::Fill),
    ]
    .spacing(5)
    .into()
}

fn xxtea_cfb_settings(state: &XxteaCfbArgs) -> Element<Message> {
    column![
        row![
            column![
                text("IV"),
                text_input("IV", state.iv.as_deref().unwrap_or(""))
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::XxteaCfb(
                            XxteaCfbSettingsMessage::IVChanged(value),
                        ))
                    })
                    .width(Length::Fill),
            ]
            .spacing(5)
            .width(Length::Fill),
            column![
                text("Block Size"),
                text_input("Block Size", state.block_size.as_deref().unwrap_or(""))
                    .on_input(|val| {
                        let value = if val.is_empty() { None } else { Some(val) };
                        Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::XxteaCfb(
                            XxteaCfbSettingsMessage::BlockSizeChanged(value),
                        ))
                    })
                    .width(100)
            ]
            .spacing(5),
        ]
        .spacing(10),
        vertical_space().height(10),
        column![
            text("Key").width(Length::Fill),
            text_input("Key", state.key.as_deref().unwrap_or(""))
                .on_input(|val| {
                    let value = if val.is_empty() { None } else { Some(val) };
                    Message::AlgorithmSettingsChanged(AlgorithmSettingsMessage::XxteaCfb(
                        XxteaCfbSettingsMessage::KeyChanged(value),
                    ))
                })
                .width(Length::Fill)
        ]
        .spacing(5),
    ]
    .into()
}
