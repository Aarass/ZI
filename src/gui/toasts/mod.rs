use iced::{
    widget::{
        button, container, horizontal_space, opaque, row, scrollable, svg, text, vertical_space,
        Column,
    },
    Background, Border, Color, Element, Length, Shadow, Theme,
};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

#[derive(Clone)]
pub struct Toast {
    pub id: usize,
    pub message: String,
    pub severity: Severity,
    pub timestamp: SystemTime,
}

#[derive(Clone, Copy)]
pub enum Severity {
    Success,
    Error,
    Info,
}

const SHORT_TOAST_DURATION: Duration = Duration::from_secs(10);
const LONG_TOAST_DURATION: Duration = Duration::from_secs(15);

use std::sync::atomic::{AtomicUsize, Ordering};

use super::state::{messages::Message, State};
static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Toast {
    pub fn expired(&self) -> bool {
        if let Ok(duration) = self.timestamp.elapsed() {
            match self.severity {
                Severity::Success => duration.ge(&SHORT_TOAST_DURATION),
                Severity::Error => duration.ge(&LONG_TOAST_DURATION),
                Severity::Info => duration.ge(&SHORT_TOAST_DURATION),
            }
        } else {
            false
        }
    }
}

pub fn push_toast(toasts: &Arc<RwLock<Vec<Toast>>>, message: &str, severity: Severity) {
    toasts.write().unwrap().push(Toast {
        id: COUNTER.fetch_add(1, Ordering::Relaxed),
        message: message.to_owned(),
        severity,
        timestamp: SystemTime::now(),
    });
}

pub fn toasts_widget(state: &State) -> Element<Message> {
    let toasts: Vec<Element<Message>> = state
        .toasts
        .read()
        .unwrap()
        .clone()
        .into_iter()
        .rev()
        .map(|toast| toast_widget(toast))
        .collect();

    scrollable(
        Column::new()
            .width(Length::Fill)
            .spacing(10)
            .padding([0, 10])
            .extend(toasts)
            .push(vertical_space().height(0)),
    )
    .anchor_bottom()
    .style(|_, _| scrollable::Style {
        container: container::Style::default(),
        vertical_rail: scrollable::Rail {
            background: None,
            border: Border::default(),
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border: Border::default(),
            },
        },
        horizontal_rail: scrollable::Rail {
            background: None,
            border: Border::default(),
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border: Border::default(),
            },
        },
        gap: None,
    })
    .into()
}

pub fn toast_widget(toast: Toast) -> Element<'static, Message> {
    let severity = toast.severity;
    let message = toast.message;

    opaque(
        container(row![
            container(text(message)).width(Length::Fill),
            horizontal_space().width(10),
            button(
                svg(svg::Handle::from_path(PathBuf::from("./assets/close.svg")))
                    .width(20)
                    .height(20)
            )
            .style(|_, _| {
                button::Style {
                    background: None,
                    text_color: Color::BLACK,
                    border: Border::default().rounded(500),
                    shadow: Shadow::default(),
                }
            })
            .padding(0)
            .width(20)
            .height(20)
            .on_press(Message::DeleteToast(toast.id))
        ])
        .width(Length::Fill)
        .padding([10, 10])
        .style(move |theme: &Theme| get_toast_style(theme, severity)),
    )
}

pub fn get_toast_style(theme: &Theme, severity: Severity) -> container::Style {
    let color = match severity {
        Severity::Success => theme.palette().success,
        Severity::Error => theme.palette().danger,
        Severity::Info => theme.palette().primary,
    };

    container::Style::default()
        .background(Background::Color(color))
        .border(Border::default().rounded(10.0))
}
