use crate::algorithms::Operation;
use crate::gui::toasts::{push_toast, toasts_widget, Severity, Toast};
use crate::hash;
use crate::utils::{get_algorithm, get_dir_path, get_file_path, get_new_file_path2, process_file};

use super::fsw_state::FSWState;
use super::manual_state::ManualState;
use super::messages::{
    AlgorithmSettingsMessage, EnigmaSettingsMesasge, FSWPageMessage, ManualPageMessage, Message,
    NavigationMessage, TcpPageMessage, XxteaCfbSettingsMesasge, XxteaSettingsMesasge,
};
use super::settings_state::SettingsState;
use super::tcp_state::{TcpMode, TcpState};

use super::super::pages::{
    fsw::fsw_page, manual::manual_page, settings::settings_page, tcp::tcp_page, Page,
};

use std::ops::Deref;
use std::time::Duration;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use iced::futures::SinkExt;
use iced::{
    alignment, stream,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, stack, svg, text, Row,
    },
    Alignment, Border, Color, Element, Length, Shadow, Subscription, Task, Theme,
};

use notify::{recommended_watcher, RecursiveMode, Watcher};

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Default)]
pub struct State {
    pub page: Page,
    pub fsw: FSWState,
    pub manual: ManualState,
    pub tcp: TcpState,
    pub settings: SettingsState,
    pub commited_settings: Arc<RwLock<SettingsState>>,

    pub toasts: Arc<RwLock<Vec<Toast>>>,
}

impl State {
    pub fn view(&self) -> Element<Message> {
        let tab_radius = 10.0;
        let navigation: Row<Message> = row![
            // horizontal_space().width(10),
            button(text("FS Watcher").align_x(alignment::Horizontal::Center))
                // .width(Length::Fill)
                .width(150)
                .on_press(Message::Navigation(NavigationMessage::GoToFSWPage))
                .style(move |theme: &Theme, status| {
                    let mut style = if let Page::Fsw = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    };

                    style.border.radius = iced::border::Radius {
                        top_left: tab_radius,
                        top_right: tab_radius,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    };

                    style
                }),
            button(text("Manual").align_x(alignment::Horizontal::Center))
                // .width(Length::Fill)
                .width(150)
                .on_press(Message::Navigation(NavigationMessage::GoToManualPage))
                .style(move |theme: &Theme, status| {
                    let mut style = if let Page::Manual = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    };

                    style.border.radius = iced::border::Radius {
                        top_left: tab_radius,
                        top_right: tab_radius,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    };

                    style
                }),
            button(text("Tcp").align_x(alignment::Horizontal::Center))
                // .width(Length::Fill)
                .width(150)
                .on_press(Message::Navigation(NavigationMessage::GoToTcpPage))
                .style(move |theme: &Theme, status| {
                    let mut style = if let Page::Tcp = self.page {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    };

                    style.border.radius = iced::border::Radius {
                        top_left: tab_radius,
                        top_right: tab_radius,
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
                let mut style = if let Page::Settings = self.page {
                    button::primary(theme, status)
                } else {
                    button::secondary(theme, status)
                };

                style.border.radius = iced::border::Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_right: 0.0,
                    bottom_left: tab_radius,
                };

                style
            }),
        ]
        .spacing(1);

        let page: Element<Message> = match self.page {
            Page::Fsw => fsw_page(self),
            Page::Manual => manual_page(self),
            Page::Tcp => tcp_page(self),
            Page::Settings => settings_page(self),
        };

        let toasts_overlay = container(row![
            horizontal_space().width(Length::FillPortion(1)),
            container(toasts_widget(self)).width(Length::FillPortion(2))
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::End);

        let main_view = column![
            navigation,
            horizontal_rule(0),
            container(page)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        ];

        stack![main_view, toasts_overlay].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(navigation_message) => match navigation_message {
                NavigationMessage::GoToFSWPage => {
                    self.page = Page::Fsw;
                    Task::none()
                }
                NavigationMessage::GoToManualPage => {
                    self.page = Page::Manual;
                    Task::none()
                }
                NavigationMessage::GoToTcpPage => {
                    self.page = Page::Tcp;
                    Task::none()
                }
                NavigationMessage::GoToSettingsPage => {
                    self.page = Page::Settings;
                    Task::none()
                }
            },
            Message::FSW(fsw_message) => match fsw_message {
                FSWPageMessage::GetDirToWatch => Task::perform(get_dir_path(), |path| {
                    Message::FSW(FSWPageMessage::DirToWatchResult(path))
                }),
                FSWPageMessage::DirToWatchResult(path_buf) => {
                    if let Some(path) = path_buf {
                        self.fsw.from = Some(path);
                    }
                    Task::none()
                }
                FSWPageMessage::GetDirToSaveTo => Task::perform(get_dir_path(), |path| {
                    Message::FSW(FSWPageMessage::DirToSaveToResult(path))
                }),
                FSWPageMessage::DirToSaveToResult(path_buf) => {
                    if let Some(path) = path_buf {
                        self.fsw.to = Some(path);
                    }
                    Task::none()
                }
                FSWPageMessage::ToggleMode => {
                    if let Operation::Encrypt = self.fsw.mode {
                        self.fsw.mode = Operation::Decrypt
                    } else {
                        self.fsw.mode = Operation::Encrypt
                    };
                    Task::none()
                }
                FSWPageMessage::TurnOn => {
                    let dir_to_watch = self
                        .fsw
                        .from
                        .as_ref()
                        .expect("This should not allow UI")
                        .to_owned();

                    let dest_dir = self
                        .fsw
                        .to
                        .as_ref()
                        .expect("This should not allow UI")
                        .to_owned();

                    if match (dir_to_watch.canonicalize(), dest_dir.canonicalize()) {
                        (Ok(canonical_path1), Ok(canonical_path2)) => {
                            canonical_path1 == canonical_path2
                        }
                        _ => false,
                    } {
                        eprintln!("Source and destination directory are the same. This would create an infinite loop.");
                        push_toast(
                                            &self.toasts,
                                            "Source and destination directory are the same. This would create an infinite loop.",
                                            Severity::Error,
                                        );
                        return Task::none();
                    };

                    let operation = self.fsw.mode.to_owned();

                    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(10);

                    let mut watcher = recommended_watcher(move |res| {
                                            match res {
                                                Ok(event) => {
                                                    event_tx.blocking_send(event).expect("Send error. A send operation can only fail if the receiving end of a channel is disconnected, implying that the data could never be received. The error contains the data being sent as a payload so it can be recovered.")
                                                },
                                                Err(e) => println!("Event handler recieved error: {:?}", e),
                                            }
                                        }).expect("Couldn't create watcher");

                    watcher
                        .watch(&dir_to_watch, RecursiveMode::NonRecursive)
                        .expect("Couldn't start watcher");

                    self.fsw.watcher = Some(Box::new(watcher));

                    let toasts = self.toasts.clone();

                    let settings_pointer = Arc::clone(&self.commited_settings);

                    tokio::spawn(async move {
                        while let Some(event) = event_rx.recv().await {
                            if !matches!(event.kind, notify::EventKind::Create(_)) {
                                continue;
                            }

                            let file_path = event.paths.first().unwrap().to_owned();
                            // println!("New File: {:?}", file_path);

                            // "Waiting" for file to become ready
                            for _ in 0..5 {
                                match tokio::fs::File::open(&file_path).await {
                                    Ok(_) => break,
                                    Err(_) => {
                                        tokio::time::sleep(Duration::from_millis(100)).await;
                                    }
                                }
                            }

                            let dest_dir = dest_dir.clone();

                            let toasts = toasts.clone();

                            let alg = match get_algorithm(&settings_pointer.read().unwrap()) {
                                Ok(a) => a,
                                Err(err) => {
                                    push_toast(&toasts, &format!("{}", err), Severity::Error);
                                    return ;
                                }
                            };

                            tokio::spawn(async move {
                                return match process_file(&file_path, &alg, operation, &dest_dir)
                                    .await
                                {
                                    Ok(_) => {
                                        push_toast(
                                            &toasts,
                                            "Successfully processed file",
                                            Severity::Success,
                                        );
                                    }
                                    Err(err) => {
                                        eprintln!(
                                            "There was an error processing the file: {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "There was an error processing the file",
                                            Severity::Error,
                                        );
                                    }
                                };
                            });
                        }
                    });

                    Task::done(Message::FSW(FSWPageMessage::WatchingStarted))
                }
                FSWPageMessage::TurnOff => {
                    let dir_to_watch = match &self.fsw.from {
                        Some(path_buff) => path_buff.to_owned(),
                        None => {
                            println!("There is no selected directory to watch");
                            return Task::none();
                        }
                    };

                    if let Some(w) = self.fsw.watcher.as_mut() {
                        if w.unwatch(&dir_to_watch).is_ok() {
                            self.fsw.watcher.take();
                            println!("Unwatched");
                        } else {
                            println!("Couldn't unwatch");
                        }
                    }

                    Task::done(Message::FSW(FSWPageMessage::WatchingEnded))
                }
                FSWPageMessage::WatchingStarted => {
                    self.fsw.is_on = true;
                    Task::none()
                }
                FSWPageMessage::WatchingEnded => {
                    self.fsw.is_on = false;
                    Task::none()
                }
            },
            Message::Manual(manual_message) => match manual_message {
                ManualPageMessage::GetFile => Task::perform(get_file_path(), |path| {
                    Message::Manual(ManualPageMessage::FileResult(path))
                }),
                ManualPageMessage::FileResult(path_buf) => {
                    if let Some(path) = path_buf {
                        self.manual.from = Some(path);
                    }
                    Task::none()
                }
                ManualPageMessage::GetDirToSaveTo => Task::perform(get_dir_path(), |path| {
                    Message::Manual(ManualPageMessage::DirToSaveToResult(path))
                }),
                ManualPageMessage::DirToSaveToResult(path_buf) => {
                    if let Some(path) = path_buf {
                        self.manual.to = Some(path);
                    }
                    Task::none()
                }
                ManualPageMessage::StartEncryption => {
                    self.manual.is_doing_work = true;

                    let file_path = self
                        .manual
                        .from
                        .to_owned()
                        .expect("UI should not allow this");

                    let dest_dir = self.manual.to.to_owned().expect("UI should not allow this");

                    let toasts = self.toasts.clone();

                    let alg = match get_algorithm(&self.commited_settings.read().unwrap()) {
                        Ok(a) => Arc::new(a),
                        Err(err) => {
                            push_toast(&toasts, &format!("{}", err), Severity::Error);
                            self.manual.is_doing_work = false;
                            return Task::none();
                        }
                    };

                    Task::perform(
                        async move {
                            match process_file(
                                &file_path,
                                alg.deref(),
                                Operation::Encrypt,
                                &dest_dir,
                            )
                            .await
                            {
                                Ok(_) => {
                                    push_toast(
                                        &toasts,
                                        "Successfully processed file",
                                        Severity::Success,
                                    );
                                }
                                Err(err) => {
                                    eprintln!("There was an error processing the file: {:?}", err);
                                    push_toast(
                                        &toasts,
                                        "There was an error processing the file",
                                        Severity::Error,
                                    );
                                }
                            }
                        },
                        |_| Message::Manual(ManualPageMessage::EncryptionDone),
                    )
                }
                ManualPageMessage::EncryptionDone => {
                    self.manual.is_doing_work = false;
                    Task::none()
                }
                ManualPageMessage::StartDecryption => {
                    self.manual.is_doing_work = true;

                    let toasts = self.toasts.clone();

                    let alg = match get_algorithm(&self.commited_settings.read().unwrap()) {
                        Ok(a) => Arc::new(a),
                        Err(err) => {
                            push_toast(&toasts, &format!("{}", err), Severity::Error);
                            self.manual.is_doing_work = false;
                            return Task::none();
                        }
                    };

                    let file_path = self
                        .manual
                        .from
                        .to_owned()
                        .expect("UI should not allow this");

                    let dest_dir = self.manual.to.to_owned().expect("UI should not allow this");

                    Task::perform(
                        async move {
                            match process_file(
                                &file_path,
                                alg.deref(),
                                Operation::Decrypt,
                                &dest_dir,
                            )
                            .await
                            {
                                Ok(_) => {
                                    push_toast(
                                        &toasts,
                                        "Successfully processed file",
                                        Severity::Success,
                                    );
                                }
                                Err(err) => {
                                    eprintln!("There was an error processing the file: {:?}", err);
                                    push_toast(
                                        &toasts,
                                        "There was an error processing the file",
                                        Severity::Error,
                                    );
                                }
                            }
                        },
                        |_| Message::Manual(ManualPageMessage::DecryptionDone),
                    )
                }
                ManualPageMessage::DecryptionDone => {
                    self.manual.is_doing_work = false;
                    Task::none()
                }
            },
            Message::Tcp(tcp_page_message) => match tcp_page_message {
                TcpPageMessage::ToggleMode => {
                    if let TcpMode::Sending = self.tcp.mode {
                        self.tcp.mode = TcpMode::Receiving
                    } else {
                        self.tcp.mode = TcpMode::Sending
                    };
                    Task::none()
                }
                TcpPageMessage::SelectFileToSend => Task::perform(get_file_path(), |path| {
                    Message::Tcp(TcpPageMessage::FileToSendResult(path))
                }),
                TcpPageMessage::FileToSendResult(path_buf) => {
                    if let Some(path) = path_buf {
                        self.tcp.file = Some(path);
                    }
                    Task::none()
                }
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
                }
                TcpPageMessage::RecieverPortChanged(val) => {
                    if val.is_empty() {
                        self.tcp.reciever_port = None
                    } else if let Ok(port) = val.parse::<u16>() {
                        self.tcp.reciever_port = Some(port);
                    }
                    Task::none()
                }
                TcpPageMessage::Send => {
                    let toasts = self.toasts.clone();
                    let address = format!(
                        "{}:{}",
                        self.tcp
                            .reciever_adress
                            .as_ref()
                            .expect("UI logic should not allow this"),
                        self.tcp
                            .reciever_port
                            .as_ref()
                            .expect("UI logic should not allow this")
                    );

                    let file_path = self
                        .tcp
                        .file
                        .as_ref()
                        .expect("UI logic should not allow this")
                        .to_owned();

                    let file_name = match file_path.file_name() {
                        Some(str) => str.to_string_lossy().to_string(),
                        None => {
                            push_toast(&toasts, "Coulnd't extract file name", Severity::Error);
                            return Task::none();
                        }
                    };

                    let alg = match get_algorithm(&self.commited_settings.read().unwrap()) {
                        Ok(a) => Arc::new(a),
                        Err(err) => {
                            push_toast(&toasts, &format!("{}", err), Severity::Success);
                            return Task::none();
                        }
                    };

                    self.tcp.is_sending = true;
                    Task::perform(
                        async move {
                            let mut file = match tokio::fs::File::open(file_path).await {
                                Ok(s) => s,
                                Err(err) => {
                                    eprintln!("Error opening the file when trying to send it over tcp: {:?}", err);
                                    push_toast(&toasts, "Couldn't open the file", Severity::Error);
                                    return ;
                                }
                            };

                            let mut file_name_leb128_buf: Vec<u8> = Vec::new();
                            let file_name_leb128_bytes = match leb128::write::unsigned(
                                &mut file_name_leb128_buf,
                                file_name.len().try_into().unwrap(),
                            ) {
                                Ok(b) => b,
                                Err(err) => {
                                    eprintln!("Error encoding file name len-prefix {:?}", err);
                                    push_toast(&toasts, "Error", Severity::Error);
                                    return ;
                                }
                            };
                            let file_name_prefix = &file_name_leb128_buf[..file_name_leb128_bytes];

                            let file_size = match file.metadata().await {
                                Ok(metadata) => metadata.len(),
                                Err(err) => {
                                    eprintln!("Error trying to read file metadata: {:?}", err);
                                    push_toast(&toasts, "Error", Severity::Error);
                                    return ;
                                }
                            };

                            let mut file_content =
                                Vec::with_capacity(file_size.try_into().unwrap());
                            match file.read_to_end(&mut file_content).await {
                                Ok(_) => (),
                                Err(err) => {
                                    eprintln!("Error reading the file when trying to send it over tcp: {:?}", err);
                                    push_toast(&toasts, "Couldn't read the file", Severity::Error);
                                    return ;
                                }
                            };

                            let encrypted_file_content = match alg.encrypt(&file_content) {
                                Ok(v) => v,
                                Err(err) => {
                                    eprintln!("Error encrypting file content: {:?}", err);
                                    push_toast(&toasts, "Error", Severity::Error);
                                    return ;
                                }
                            };

                            let hash = hash::hash_data(&encrypted_file_content);

                            match TcpStream::connect(address).await {
                                Ok(mut stream) => {
                                    println!("Successfully connected to the server");
                                    push_toast(&toasts, "Established connection", Severity::Info);
                                    match stream.write_all(file_name_prefix).await {
                                        Ok(_) => {
                                            println!(
                                                "Successfully sent len-prefix of the file name"
                                            )
                                        }
                                        Err(err) => {
                                            eprintln!(
                                                "Error sending len-prefix of the file name: {:?}",
                                                err
                                            );
                                            push_toast(
                                                &toasts,
                                                "An error occurred while sending data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    };

                                    match stream.write_all(file_name.as_bytes()).await {
                                        Ok(_) => {
                                            println!("Successfully sent file name: {}", file_name);
                                        }
                                        Err(err) => {
                                            eprintln!("Error sending file name : {:?}", err);
                                            push_toast(
                                                &toasts,
                                                "An error occurred while sending data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    };

                                    match stream.write_i64_le(file_size.try_into().unwrap()).await {
                                        Ok(_) => {
                                            println!("Successfully sent file size: {}", file_size);
                                        }
                                        Err(err) => {
                                            eprintln!("Error sending file size : {:?}", err);
                                            push_toast(
                                                &toasts,
                                                "An error occurred while sending data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    };

                                    match stream.write_i32_le(hash.len().try_into().unwrap()).await
                                    {
                                        Ok(_) => {
                                            println!("Succesffully sent hash size {}", hash.len());
                                        }
                                        Err(err) => {
                                            eprintln!("Error sending hash size : {:?}", err);
                                            push_toast(
                                                &toasts,
                                                "An error occurred while sending data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    }

                                    match stream.write_all(&hash).await {
                                        Ok(_) => {
                                            println!(
                                                "Successfully sent hash:\n {}",
                                                String::from_utf8_lossy(&hash)
                                            );
                                        }
                                        Err(err) => {
                                            eprintln!("Error sending hash: {:?}", err);
                                            push_toast(
                                                &toasts,
                                                "An error occurred while sending data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    }

                                    match stream.write_all(&encrypted_file_content).await {
                                        Ok(_) => {
                                            println!(
                                                "Successfully sent encrypted file content:\n {:?}",
                                                String::from_utf8_lossy(&encrypted_file_content)
                                            )
                                        }
                                        Err(err) => {
                                            eprintln!("Error sending file content: {:?}", err);
                                            push_toast(
                                                &toasts,
                                                "An error occurred while sending data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    };

                                    push_toast(
                                        &toasts,
                                        "The file was successfully sent",
                                        Severity::Success,
                                    );

                                    match stream.shutdown().await {
                                        Ok(_) => {
                                            println!("Successfully closed connection");
                                        }
                                        Err(err) => {
                                            eprintln!(
                                                                "An error occurred while closing the connection: {:?}",
                                                                err
                                                            );
                                            push_toast(
                                                &toasts,
                                                "An error occurred while closing the connection",
                                                Severity::Error,
                                            );
                                        }
                                    };
                                }
                                Err(err) => {
                                    println!("Error opening connection to the server {:?}", err);
                                    push_toast(
                                        &toasts,
                                        "Failed to establish a connection",
                                        Severity::Error,
                                    );
                                    return ;
                                }
                            };
                        },
                        |_| Message::Tcp(TcpPageMessage::Sent),
                    )
                }
                TcpPageMessage::Sent => {
                    self.tcp.is_sending = false;
                    Task::none()
                }
                TcpPageMessage::SelectDirToStoreFiles => Task::perform(get_dir_path(), |path| {
                    Message::Tcp(TcpPageMessage::DirToStoreFilesResult(path))
                }),
                TcpPageMessage::DirToStoreFilesResult(path_buf) => {
                    if let Some(path) = path_buf {
                        self.tcp.dir_to_store_files = Some(path);
                    }
                    Task::none()
                }
                TcpPageMessage::MyPortChanged(val) => {
                    if val.is_empty() {
                        self.tcp.my_port = None
                    } else if let Ok(port) = val.parse::<u16>() {
                        self.tcp.my_port = Some(port);
                    }
                    Task::none()
                }
                TcpPageMessage::StartListening => {
                    let toasts = self.toasts.clone();

                    // TODO
                    // UI should not allow this
                    let my_port = self
                        .tcp
                        .my_port
                        .expect("My port is none when trying to start tcp server");

                    let dest_dir = self
                        .tcp
                        .dir_to_store_files
                        .clone()
                        .expect("Dest dir is none when trying to start tcp server");

                    let settings_pointer = self.commited_settings.clone();

                    let handle = tokio::spawn(async move {
                        let listener = match TcpListener::bind(format!("127.0.0.1:{:?}", my_port))
                            .await
                        {
                            Ok(listener) => {
                                println!("Successfully started tcp server");
                                listener
                            }
                            Err(err) => {
                                println!("Couldn't start listening: {:?}", err);
                                push_toast(&toasts, "Couldn't start listening", Severity::Error);
                                return ;
                            }
                        };

                        loop {
                            let (socket, addr) = match listener.accept().await {
                                Ok(val) => val,
                                Err(err) => {
                                    println!("Error accepting tcp connection: {:?}", err);
                                    push_toast(&toasts, "Faulty connection", Severity::Error);
                                    continue;
                                }
                            };

                            let message = format!("Accepted connection with: {:?}", addr);
                            println!("{}", message);
                            push_toast(&toasts, &message, Severity::Info);

                            let toasts = toasts.clone();

                            let alg = match get_algorithm(&settings_pointer.read().unwrap()) {
                                Ok(a) => Arc::new(a),
                                Err(err) => {
                                    push_toast(&toasts, &format!("{}", err), Severity::Success);
                                    return;
                                }
                            };

                            let dest_dir = dest_dir.clone();
                            tokio::spawn(async move {
                                let mut socket = BufReader::new(socket);
                                match socket.fill_buf().await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        println!("Error filling buffer from tcp stream: {:?}", err);
                                        push_toast(
                                            &toasts,
                                            "An error occurred while fetching data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };

                                let file_name_len = {
                                    let mut buf = socket.buffer();
                                    let file_name_len = match leb128::read::unsigned(&mut buf) {
                                        Ok(v) => v,
                                        Err(err) => {
                                            eprintln!("Error occured while extracting len-prefix of file name: {:?}", err);
                                            push_toast(
                                                &toasts,
                                                "An error occurred while extracting data",
                                                Severity::Error,
                                            );
                                            return ;
                                        }
                                    };

                                    let bytes_to_consume = socket.buffer().len() - buf.len();
                                    socket.consume(bytes_to_consume);

                                    file_name_len
                                };
                                println!("Length-prefix: {:?}", file_name_len);

                                let mut file_name_buf =
                                    vec![0u8; file_name_len.try_into().unwrap()];
                                match socket.read_exact(&mut file_name_buf).await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!(
                                            "Error occured while extracting file name {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "An error occurred while extracting data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };

                                let file_name = match std::str::from_utf8(&file_name_buf) {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!(
                                            "Error occured while extracting file name {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "An error occurred while extracting data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };
                                println!("File name: {:}", file_name);

                                let file_len = match socket.read_i64_le().await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!(
                                            "Error occured while extracting file length {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "An error occurred while extracting data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };
                                println!("Content lenght: {:}", file_len);

                                let hash_len = match socket.read_i32_le().await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!(
                                            "Error occured while extracting hash length {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "An error occurred while extracting data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };
                                println!("Hash lenght: {:}", hash_len);

                                let mut hash_buffer = vec![0_u8; hash_len.try_into().unwrap()];
                                match socket.read_exact(&mut hash_buffer).await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!("Error occured while extracting hash {:?}", err);
                                        push_toast(
                                            &toasts,
                                            "An error occurred while extracting data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };

                                let hash = String::from_utf8_lossy(&hash_buffer);
                                println!("Recieved hash: {:}", hash);

                                let mut encrypted_content = Vec::new();
                                match socket.read_to_end(&mut encrypted_content).await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!(
                                            "Error occured while extracting encrypted content {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "An error occurred while extracting data",
                                            Severity::Error,
                                        );
                                        return ;
                                    }
                                };

                                match socket.shutdown().await {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!(
                                            "An error occurred while closing the connection: {:?}",
                                            err
                                        );
                                        push_toast(
                                            &toasts,
                                            "An error occurred while closing the connection",
                                            Severity::Error,
                                        );
                                    }
                                };

                                drop(socket);

                                println!(
                                    "Recived encrypted data:\n {:?}",
                                    String::from_utf8_lossy(&encrypted_content)
                                );

                                let decrypted_file_content = match alg.decrypt(&encrypted_content) {
                                    Ok(v) => v,
                                    Err(err) => {
                                        eprintln!("Error decrypting file content: {:?}", err);
                                        push_toast(&toasts, "Error", Severity::Error);
                                        return ;
                                    }
                                };

                                println!(
                                    "Decrypted data:\n {:?}",
                                    String::from_utf8_lossy(&decrypted_file_content)
                                );

                                let new_file_path = match get_new_file_path2(
                                    file_name,
                                    &dest_dir,
                                    Operation::Decrypt,
                                )
                                .await
                                {
                                    Ok(val) => val,
                                    Err(_) => {
                                        push_toast(
                                            &toasts,
                                            "Couldn't find available name for the file",
                                            Severity::Error,
                                        );
                                        return;
                                    }
                                };

                                let mut new_file =
                                    match tokio::fs::File::create(new_file_path).await {
                                        Ok(val) => val,
                                        Err(_) => {
                                            push_toast(
                                                &toasts,
                                                "Couldn't create file to store result into",
                                                Severity::Error,
                                            );
                                            return;
                                        }
                                    };

                                match new_file.write_all(&decrypted_file_content).await {
                                    Ok(_) => (),
                                    Err(_) => {
                                        push_toast(
                                            &toasts,
                                            "Error while writing result to the file",
                                            Severity::Error,
                                        );
                                        return;
                                    }
                                };

                                push_toast(
                                    &toasts,
                                    "Successfully recieved a file over the tcp",
                                    Severity::Success,
                                );
                            });
                        }
                    });

                    self.tcp.join_handle = Some(handle);

                    self.tcp.is_listening = true;
                    Task::none()
                }
                TcpPageMessage::StopListening => {
                    if let Some(handle) = self.tcp.join_handle.take() {
                        handle.abort();
                        println!("Successfully stoped listening for tcp connections");
                    } else {
                        println!("There is no join handle in state")
                    }

                    self.tcp.is_listening = false;
                    Task::none()
                }
            },
            Message::AlgorithmChanged(new_option) => {
                self.settings.algorithm_option = new_option;
                Task::none()
            }
            Message::AlgorithmSettingsChanged(algorithm_settings_message) => {
                match algorithm_settings_message {
                    AlgorithmSettingsMessage::Enigma(enigma_settings_mesasge) => {
                        match enigma_settings_mesasge {
                            EnigmaSettingsMesasge::ReflWiringChanged(value) => {
                                self.settings.enigma_args.refl_wiring = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot1WiringChanged(value) => {
                                self.settings.enigma_args.rot1_wiring = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot1NotchChanged(value) => {
                                self.settings.enigma_args.rot1_notch = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot1PositionChanged(value) => {
                                self.settings.enigma_args.rot1_position = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot2WiringChanged(value) => {
                                self.settings.enigma_args.rot2_wiring = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot2NotchChanged(value) => {
                                self.settings.enigma_args.rot2_notch = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot2PositionChanged(value) => {
                                self.settings.enigma_args.rot2_position = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot3WiringChanged(value) => {
                                self.settings.enigma_args.rot3_wiring = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot3NotchChanged(value) => {
                                self.settings.enigma_args.rot3_notch = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::Rot3PositionChanged(value) => {
                                self.settings.enigma_args.rot3_position = value;
                                Task::none()
                            }
                            EnigmaSettingsMesasge::PlugboardChanged(value) => {
                                self.settings.enigma_args.plugboard = value;
                                Task::none()
                            }
                        }
                    }
                    AlgorithmSettingsMessage::Xxtea(xxteasettings_mesasge) => {
                        match xxteasettings_mesasge {
                            XxteaSettingsMesasge::KeyChanged(value) => {
                                self.settings.xxtea_args.key = value;
                                Task::none()
                            }
                        }
                    }
                    AlgorithmSettingsMessage::XxteaCfb(xxtea_cfbsettings_mesasge) => {
                        match xxtea_cfbsettings_mesasge {
                            XxteaCfbSettingsMesasge::KeyChanged(value) => {
                                self.settings.xxtea_cfb_args.key = value;
                                Task::none()
                            }
                            XxteaCfbSettingsMesasge::IVChanged(value) => {
                                self.settings.xxtea_cfb_args.iv = value;
                                Task::none()
                            }
                            XxteaCfbSettingsMesasge::BlockSizeChanged(value) => {
                                self.settings.xxtea_cfb_args.block_size = value;
                                Task::none()
                            }
                        }
                    }
                }
            }
            Message::CommitSettings => {
                self.commit_settings();
                Task::none()
            }
            Message::DeleteToast(id) => {
                let index = self
                    .toasts
                    .read()
                    .unwrap()
                    .iter()
                    .position(|val| val.id == id)
                    .expect("Bug!!! Expect a valid id to be passed");

                self.toasts.write().unwrap().remove(index);

                Task::none()
            }
            Message::Tick => {
                let unfiltered = self.toasts.read().unwrap();
                let unfiltered_len = unfiltered.len();

                let filtered: Vec<Toast> = unfiltered
                    .iter()
                    .filter(|toast| !toast.expired())
                    .cloned()
                    .collect();
                let filtered_len = filtered.len();

                drop(unfiltered);

                if filtered_len != unfiltered_len {
                    let mut ts = self.toasts.write().unwrap();
                    ts.clear();
                    ts.extend(filtered);
                }

                Task::none()
            }
            Message::Empty => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            stream::channel(100, |mut output| async move {
                output.send(()).await.unwrap();
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    output.send(()).await.unwrap();
                }
            })
        })
        .map(|_| Message::Tick)
    }

    fn commit_settings(&self) {
        match self.commited_settings.write() {
            Ok(mut write_handle) => {
                *write_handle = self.settings.clone();
            }
            Err(err) => {
                push_toast(&self.toasts, &format!("{}", err), Severity::Error);
            }
        }
    }
}
