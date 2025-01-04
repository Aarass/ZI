use tokio::sync::oneshot;
use tokio::sync::mpsc;
use std::path::PathBuf;
use rfd::AsyncFileDialog;
use iced::{alignment, widget::{button, column, container, horizontal_space, pick_list, row, text, text_input, toggler, vertical_space, Column, Row }, Element, Length, Size, Task, Theme };
use notify::{recommended_watcher, RecursiveMode, Watcher};
use tokio::time::{sleep, Duration};

async fn fake_work() -> i32 {
    sleep(Duration::from_secs(2)).await;
    1
}

#[tokio::main]
async fn main() -> std::result::Result<(), iced::Error> {
    iced::application("ZI", State::update, State::view)
        .theme(|_| iced::Theme::Dark)
        .window_size(Size::new(600.0, 400.0))
        .centered()
        .run()
}

#[derive(Default)]
struct State {
    page: Page,
    fsw: FSWState,
    manual: ManualState,
    tcp: TcpState,
    algoritham: String,
    key: String,

    watcher: Option<Box<dyn Watcher>>,
    end_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

#[derive(Default)]
struct FSWState {
    from: Option<PathBuf>,
    to: Option<PathBuf>,
    is_on: bool
}

#[derive(Default)]
struct ManualState {
    from: Option<PathBuf>,
    to: Option<PathBuf>,
    is_doing_work: bool
}

#[derive(Default)]
struct TcpState {
    mode: TcpMode,
    //------------------------------------------
    file: Option<PathBuf>,
    reciever_adress: Option<String>,
    reciever_port: Option<u32>,
    is_sending: bool,
    //------------------------------------------
    dir_to_store_files: Option<PathBuf>,
    my_port: Option<u32>,
    is_listening: bool,
}

#[derive(Default)]
enum TcpMode {
    #[default]
    Sending,
    Receiving
}


#[derive(Default)]
enum Page {
    Settings,
    #[default]
    Fsw,
    Manual,
    Tcp,
}


#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),
    FSW(FSWPageMessage),
    Manual(ManualPageMessage),
    Tcp(TcpPageMessage),
    AlgorithamChanged(String),
    KeyChanged(String),
}

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    GoToFSWPage,
    GoToManualPage,
    GoToTcpPage,
    GoToSettingsPage,
}

#[derive(Debug, Clone)]
pub enum FSWPageMessage {
    GetDirToWatch,
    DirToWatchResult(Option<PathBuf>),
    GetDirToSaveTo,
    DirToSaveToResult(Option<PathBuf>),
    TurnOn,
    TurnOff,
    WatchingStarted,
    WatchingEnded,
}

#[derive(Debug, Clone)]
pub enum ManualPageMessage {
    GetFile,
    FileResult(Option<PathBuf>),
    GetDirToSaveTo,
    DirToSaveToResult(Option<PathBuf>),
    StartEncryption,
    EncryptionDone,
    StartDecryption,
    DecryptionDone,
}

#[derive(Debug, Clone)]
pub enum TcpPageMessage {
    ToggleMode,
    SelectFileToSend,
    FileToSendResult(Option<PathBuf>),
    RecieverAddressChanged(String),
    RecieverPortChanged(String),
    Send,
    Sent,
    //--------------------------------------
    SelectDirToStoreFiles,
    DirToStoreFilesResult(Option<PathBuf>),
    MyPortChanged(String),
    StartListening,
    StopListening,
}

impl State {
    pub fn view(&self) -> Column<Message> {
        let navigation: Row<Message> = row![
            button(text("FS Watcher").align_x(alignment::Horizontal::Center)).width(Length::Fill).on_press(Message::Navigation(NavigationMessage::GoToFSWPage))
                .style(|theme: &Theme, status| {
                    if let Page::Fsw = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                }),
            button(text("Manual").align_x(alignment::Horizontal::Center)).width(Length::Fill).on_press(Message::Navigation(NavigationMessage::GoToManualPage))
                .style(|theme: &Theme, status| {
                    if let Page::Manual = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                }),
            button(text("Tcp").align_x(alignment::Horizontal::Center)).width(Length::Fill).on_press(Message::Navigation(NavigationMessage::GoToTcpPage))
                .style(|theme: &Theme, status| {
                    if let Page::Tcp = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                }),
            button(text("Settings").align_x(alignment::Horizontal::Center)).width(Length::Fill).on_press(Message::Navigation(NavigationMessage::GoToSettingsPage))
                .style(|theme: &Theme, status| {
                    if let Page::Settings = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                }),
        ];

        let page: Element<Message> = match self.page {
            Page::Fsw => fsw_page(self),
            Page::Manual => manual_page(self),
            Page::Tcp => tcp_page(self),
            Page::Settings => settings_page(self),
        };

        column![
            navigation,
            container(page).center_x(Length::Fill).center_y(Length::Fill),
        ]
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(navigation_message) => {
                match navigation_message {
                    NavigationMessage::GoToFSWPage => { self.page = Page::Fsw; Task::none() },
                    NavigationMessage::GoToManualPage => { self.page = Page::Manual; Task::none() },
                    NavigationMessage::GoToTcpPage => { self.page = Page::Tcp; Task::none() },
                    NavigationMessage::GoToSettingsPage => { self.page = Page::Settings; Task::none() },
                }
            }
            Message::FSW(fsw_message) => {
                match fsw_message {
                    FSWPageMessage::GetDirToWatch => {
                        Task::perform(get_dir_path(),|path| Message::FSW(FSWPageMessage::DirToWatchResult(path)))
                    },
                    FSWPageMessage::DirToWatchResult(path_buf) => {
                        if let Some(path) = path_buf {
                            self.fsw.from = Some(path);
                        }
                        Task::none()
                    },
                    FSWPageMessage::GetDirToSaveTo => {
                        Task::perform(get_dir_path(),|path| Message::FSW(FSWPageMessage::DirToSaveToResult(path)))
                    },
                    FSWPageMessage::DirToSaveToResult(path_buf) => {
                        if let Some(path) = path_buf {
                            self.fsw.to = Some(path);
                        }
                        Task::none()
                    },
                    FSWPageMessage::TurnOn => {
                        let dir_to_watch = match &self.fsw.from {
                            Some(path_buff) => path_buff.to_owned(),
                            None => {
                                println!("There is no selected directory to watch");
                                return Task::none()
                            },
                        };

                        let (end_tx, end_rx) = oneshot::channel::<()>();
                        let (event_tx, mut event_rx) = mpsc::channel(10);

                        let mut watcher = recommended_watcher(move |res| {
                            match res {
                                Ok(event) => {
                                    match event_tx.blocking_send(event){
                                        Ok(_) => println!("Successfully sent."),
                                        Err(_) => println!("Send error. A send operation can only fail if the receiving end of a channel is disconnected, implying that the data could never be received. The error contains the data being sent as a payload so it can be recovered."),
                                    }
                                },
                                Err(e) => println!("Event handler recieved error: {:?}", e),
                            }
                        }).expect("Couldn't create watcher");

                        watcher.watch(&dir_to_watch, RecursiveMode::NonRecursive).expect("Couldn't start watcher");

                        self.watcher = Some(Box::new(watcher));
                        self.end_tx = Some(end_tx);

                        tokio::spawn(async move {
                            while let Some(event) = event_rx.recv().await {
                                println!("{:?}", event)
                                // tokio::spawn(async move { process(event) });
                            };
                            println!("Channel closed")
                        });

                        // tokio::spawn(async move {
                        //     tokio::select! {
                        //         _ = async {
                        //             while let Some(event) = event_rx.recv().await {
                        //                 println!("{:?}", event)
                        //                 // tokio::spawn(async move { process(event) });
                        //             };
                        //         } => {}
                        //         _ = end_rx => {
                        //             println!("Recieved end signal");
                        //         }
                        //     }
                        // });

                        Task::done(Message::FSW(FSWPageMessage::WatchingStarted))
                    },
                    FSWPageMessage::TurnOff => {
                        let dir_to_watch = match &self.fsw.from {
                            Some(path_buff) => path_buff.to_owned(),
                            None => {
                                println!("There is no selected directory to watch");
                                return Task::none()
                            },
                        };

                        if let Some(w) = self.watcher.as_mut() {
                            if w.unwatch(&dir_to_watch).is_ok() {
                                self.watcher.take();
                                println!("Unwatched");
                            } else {
                                println!("Couldn't unwatch");
                            }
                        }

                        // if let Some(tx) = self.end_tx.take() {
                        //     match tx.send(()) {
                        //         Ok(_) => (),
                        //         Err(_) => {
                        //             println!("Couldn't send end signal")
                        //         },
                        //     }
                        // }

                        Task::done(Message::FSW(FSWPageMessage::WatchingEnded))
                    },
                    FSWPageMessage::WatchingStarted => {
                        self.fsw.is_on = true;
                        Task::none()
                    },
                    FSWPageMessage::WatchingEnded => {
                        self.fsw.is_on = false;
                        Task::none()
                    },
                }
            }
            Message::Manual(manual_message) => {
                match manual_message {
                    ManualPageMessage::GetFile => {
                        Task::perform(get_file_path(),|path| Message::Manual(ManualPageMessage::FileResult(path)))
                    },
                    ManualPageMessage::FileResult(path_buf) => {
                        if let Some(path) = path_buf {
                            self.manual.from = Some(path);
                        }
                        Task::none()
                    },
                    ManualPageMessage::GetDirToSaveTo => {
                        Task::perform(get_dir_path(),|path| Message::Manual(ManualPageMessage::DirToSaveToResult(path)))
                    },
                    ManualPageMessage::DirToSaveToResult(path_buf) => {
                        if let Some(path) = path_buf {
                            self.manual.to = Some(path);
                        }
                        Task::none()
                    },
                    ManualPageMessage::StartEncryption => {
                        self.manual.is_doing_work = true;
                        Task::done(Message::Manual(ManualPageMessage::EncryptionDone))
                    },
                    ManualPageMessage::EncryptionDone => {
                        self.manual.is_doing_work = false;
                        Task::none()
                    },
                    ManualPageMessage::StartDecryption => {
                        self.manual.is_doing_work = true;
                        Task::done(Message::Manual(ManualPageMessage::DecryptionDone))
                    },
                    ManualPageMessage::DecryptionDone => {
                        self.manual.is_doing_work = false;
                        Task::none()
                    },
                }
            },
            Message::Tcp(tcp_page_message) => {
                match tcp_page_message {
                    TcpPageMessage::ToggleMode => {
                        if let TcpMode::Sending = self.tcp.mode {
                            self.tcp.mode = TcpMode::Receiving
                        } else {
                            self.tcp.mode = TcpMode::Sending
                        };
                        Task::none()
                    },
                    TcpPageMessage::SelectFileToSend => {
                        Task::perform(get_file_path(),|path| Message::Tcp(TcpPageMessage::FileToSendResult(path)))
                    },
                    TcpPageMessage::FileToSendResult(path_buf) => {
                        if let Some(path) = path_buf {
                            self.tcp.file = Some(path);
                        }
                        Task::none()
                    },
                    TcpPageMessage::RecieverAddressChanged(val) => {
                        if let Some(c) = val.chars().last() {
                            if !(c.is_numeric() || c == '.') {
                                return Task::none();
                            }
                        }
                        if val.is_empty() {
                            self.tcp.reciever_adress = None
                        } else {
                            self.tcp.reciever_adress = Some(val);
                        }
                        Task::none()
                    },
                    TcpPageMessage::RecieverPortChanged(val) => {
                        if val.is_empty() {
                            self.tcp.reciever_port = None
                        } else if let Ok(port) = val.parse::<u32>() {
                            self.tcp.reciever_port = Some(port);
                        } 
                        Task::none()
                    },
                    TcpPageMessage::Send => {
                        self.tcp.is_sending = true;
                        // TODO
                        Task::perform(fake_work(),|_| Message::Tcp(TcpPageMessage::Sent))
                    },
                    TcpPageMessage::Sent => {
                        self.tcp.is_sending = false;
                        // TODO
                        Task::none()
                    },
                    TcpPageMessage::SelectDirToStoreFiles => {
                        Task::perform(get_dir_path(),|path| Message::Tcp(TcpPageMessage::DirToStoreFilesResult(path)))
                    },
                    TcpPageMessage::DirToStoreFilesResult(path_buf) => {
                        if let Some(path) = path_buf {
                            self.tcp.dir_to_store_files = Some(path);
                        }
                        Task::none()
                    },
                    TcpPageMessage::MyPortChanged(val) => {
                        if val.is_empty() {
                            self.tcp.my_port = None
                        } else if let Ok(port) = val.parse::<u32>() {
                            self.tcp.my_port = Some(port);
                        } 
                        Task::none()
                    },
                    TcpPageMessage::StartListening => {
                        self.tcp.is_listening = true;
                        // TODO
                        Task::none()
                    },
                    TcpPageMessage::StopListening => {
                        self.tcp.is_listening = false;
                        // TODO
                        Task::none()
                    },
                }
            },
            Message::AlgorithamChanged(val) => {
                self.algoritham = val;
                Task::none()
            },
            Message::KeyChanged(val) => {
                self.key = val;
                Task::none()
            },
        }
    }
}

async fn get_file_path() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_directory("/")
        .pick_file()
        .await
        .map(|fh| fh.path().to_owned())
}

async fn get_dir_path() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_directory("/")
        .pick_folder()
        .await
        .map(|fh| fh.path().to_owned())
}

fn fsw_page(state: &State) -> Element<Message> {
    let can_run = state.fsw.from.is_some() && state.fsw.to.is_some();
    let from = state.fsw.from.as_ref().map(|path| path.clone().into_os_string().into_string().unwrap()).unwrap_or(String::from(""));
    let to = state.fsw.to.as_ref().map(|path| path.clone().into_os_string().into_string().unwrap()).unwrap_or(String::from(""));
    column![
        text("Directory which the file watcher will monitor"),
        row![
            text_input("Click the \"Choose\" button", &from)
                .width(Length::Fill),
            button(text("Choose")
                .align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !state.fsw.is_on {Some(Message::FSW(FSWPageMessage::GetDirToWatch))} else {None})
        ],
        horizontal_space().height(10),
        text("Directory where the result will be saved"),
        row![
            text_input("Click the \"Choose\" button", &to)
                .width(Length::Fill),
            button(
                text("Choose")
                    .align_x(alignment::Horizontal::Center)
            )
                .width(Length::Shrink)
                .on_press_maybe(if !state.fsw.is_on {Some(Message::FSW(FSWPageMessage::GetDirToSaveTo))} else {None})
        ],
        horizontal_space().height(10),
        container(
            button(
                text(if state.fsw.is_on {"Turn off"} else {"Turn on"})
                    .align_x(alignment::Horizontal::Center)
            )
                .width(Length::Shrink)
                .on_press_maybe(
                    if can_run {
                        Some(if state.fsw.is_on { Message::FSW(FSWPageMessage::TurnOff) } else { Message::FSW(FSWPageMessage::TurnOn) })
                    } else {
                        None
                    }
                )
        ).width(Length::Fill).align_x(alignment::Horizontal::Center)
    ].padding([50, 100]).into()
}

fn manual_page(state: &State) -> Element<Message> {
    let can_run = state.manual.from.is_some() && state.manual.to.is_some() && !state.manual.is_doing_work;
    let from = state.manual.from.as_ref().map(|path| path.clone().into_os_string().into_string().unwrap()).unwrap_or(String::from(""));
    let to = state.manual.to.as_ref().map(|path| path.clone().into_os_string().into_string().unwrap()).unwrap_or(String::from(""));
    column![
        text("File to encrypt"),
        row![
            text_input("Click the \"Choose\" button", &from)
                .width(Length::Fill),
            button(text("Choose")
                .align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !state.manual.is_doing_work {Some(Message::Manual(ManualPageMessage::GetFile))} else {None})
        ],
        horizontal_space().height(10),
        text("Directory where the result will be saved"),
        row![
            text_input("Click the \"Choose\" button", &to)
                .width(Length::Fill),
            button(
                text("Choose")
                    .align_x(alignment::Horizontal::Center)
            )
                .width(Length::Shrink)
                .on_press_maybe(if !state.manual.is_doing_work {Some(Message::Manual(ManualPageMessage::GetDirToSaveTo))} else {None})
        ],
        horizontal_space().height(10),
        container(
            row![
                button(
                    text("Encrypt")
                        .align_x(alignment::Horizontal::Center)
                )
                    .width(Length::Shrink)
                    .on_press_maybe(
                        if can_run {
                            Some(Message::Manual(ManualPageMessage::StartEncryption))
                        } else {
                            None
                        }
                    ),
                horizontal_space().width(10),
                button(
                    text("Decrypt")
                        .align_x(alignment::Horizontal::Center)
                )
                    .width(Length::Shrink)
                    .on_press_maybe(
                        if can_run {
                            Some(Message::Manual(ManualPageMessage::StartDecryption))
                        } else {
                            None
                        }
                    )
            ]
        ).width(Length::Fill).align_x(alignment::Horizontal::Center)
    ].padding([50, 100]).into()
}


fn tcp_page(state: &State) -> Element<Message> {
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
        ].align_y(alignment::Vertical::Center),
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

fn valid_address(address: &Option<String>) -> bool {
    if address.is_none() {
        return false;
    }
    true
}

fn valid_port(port: &Option<u32>) -> bool {
    if port.is_none() {
        return false;
    }
    true
}

fn tcp_send_widget(state: &State) -> Element<Message> {
    let file = state.tcp.file.as_ref().map(|path| path.clone().into_os_string().into_string().unwrap()).unwrap_or(String::from(""));
    let address = state.tcp.reciever_adress.as_deref().unwrap_or("");
    let port = state.tcp.reciever_port.map(|val| val.to_string()).unwrap_or(String::from(""));
    let is_sending = state.tcp.is_sending;
    let can_send = valid_address(&state.tcp.reciever_adress) && valid_port(&state.tcp.reciever_port) && !is_sending;
    column![
        text("File to send"),
        row![
            text_input("Click the \"Choose\" button", &file)
                .width(Length::Fill),
            button(text("Choose")
                .align_x(alignment::Horizontal::Center))
                .width(Length::Shrink)
                .on_press_maybe(if !is_sending {Some(Message::Tcp(TcpPageMessage::SelectFileToSend))} else {None}),
        ],
        vertical_space().height(10),
        row![
            text_input("Address", address)
                .on_input_maybe(if !is_sending {
                    Some(|value| Message::Tcp(TcpPageMessage::RecieverAddressChanged(value)))
                } else { None })
                .width(Length::Fill),
            text(" : "),
            text_input("Port", &port)
                .on_input_maybe(if !is_sending {
                    Some(|value| Message::Tcp(TcpPageMessage::RecieverPortChanged(value)))
                } else { None })
                .width(Length::Fill),
        ].align_y(alignment::Vertical::Center),
        vertical_space().height(10),
        container(
            button(
                text(
                    if state.tcp.is_sending {"Sending..."} else {"Send"}
                )
                    .align_x(alignment::Horizontal::Center)
            )
                .width(Length::Shrink)
                .on_press_maybe(
                    if can_send {
                        Some(Message::Tcp(TcpPageMessage::Send))
                    } else {
                        None
                    }
                )
        )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
    ].into()

}

fn tcp_recieve_widget(state: &State) -> Element<Message> {
    let port = state.tcp.my_port.map(|val| val.to_string()).unwrap_or(String::from(""));
    let is_listening = state.tcp.is_listening;
    let to = state.tcp.dir_to_store_files.as_ref().map(|path| path.clone().into_os_string().into_string().unwrap()).unwrap_or(String::from(""));
    let can_start_listening = valid_port(&state.tcp.my_port) && !is_listening;
    column![
        text("Directory to save recieved files to"),
        row![
            text_input("Click the \"Choose\" button", &to)
                .width(Length::Fill),
            button(
                text("Choose")
                    .align_x(alignment::Horizontal::Center)
            )
                .width(Length::Shrink)
                .on_press_maybe(if !state.manual.is_doing_work {Some(Message::Tcp(TcpPageMessage::SelectDirToStoreFiles))} else {None})
        ],
        vertical_space().height(10),
        row![
            text_input("localhost", "").width(Length::Fill),
            text(" : "),
            text_input("Port", &port)
                .on_input_maybe(if !is_listening {
                    Some(|value| Message::Tcp(TcpPageMessage::MyPortChanged(value)))
                } else { None })
                .width(Length::Fill),
        ].align_y(alignment::Vertical::Center),
        vertical_space().height(10),
        container(
            button(
                text(if is_listening {"Stop listening"} else {"Start Listening"})
                    .align_x(alignment::Horizontal::Center)
            )
                .width(Length::Shrink)
                .on_press_maybe(
                    if can_start_listening {
                        Some(Message::Tcp(TcpPageMessage::StartListening))
                    } else if is_listening {
                        Some(Message::Tcp(TcpPageMessage::StopListening))
                    } else {
                        None
                    }
                )
        )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
    ].into()
}

fn settings_page(state: &State) -> Element<Message> {
    column![
        text_input("Key", &state.key)
            .on_input(Message::KeyChanged)
            .width(Length::Fill),
        vertical_space().height(10),
        row![
            text("Encryption/decryption algoritham: "),
            pick_list(vec![String::from("Alg1"), String::from("Alg2")], Some(state.algoritham.clone()), Message::AlgorithamChanged),
        ].align_y(alignment::Vertical::Center)
    ]
        .padding([50, 100])
        .align_x(alignment::Horizontal::Center).into()
}