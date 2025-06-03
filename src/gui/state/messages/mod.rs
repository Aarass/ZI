use std::path::PathBuf;

use crate::algorithms::AlgorithmOption;

#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),
    FSW(FSWPageMessage),
    Manual(ManualPageMessage),
    Tcp(TcpPageMessage),
    AlgorithmChanged(AlgorithmOption),
    AlgorithmSettingsChanged(AlgorithmSettingsMessage),
    CommitSettings,
    DeleteToast(usize),
    Tick,
    Empty,
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
    ToggleMode,
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

#[derive(Debug, Clone)]
pub enum AlgorithmSettingsMessage {
    Enigma(EnigmaSettingsMessage),
    Xxtea(XxteaSettingsMessage),
    XxteaCfb(XxteaCfbSettingsMessage),
}

#[derive(Debug, Clone)]
pub enum EnigmaSettingsMessage {
    ReflWiringChanged(Option<String>),
    Rot1WiringChanged(Option<String>),
    Rot1NotchChanged(Option<String>),
    Rot1RingstellungChanged(Option<String>),
    Rot1PositionChanged(Option<String>),
    Rot2WiringChanged(Option<String>),
    Rot2NotchChanged(Option<String>),
    Rot2RingstellungChanged(Option<String>),
    Rot2PositionChanged(Option<String>),
    Rot3WiringChanged(Option<String>),
    Rot3NotchChanged(Option<String>),
    Rot3RingstellungChanged(Option<String>),
    Rot3PositionChanged(Option<String>),
    PlugboardChanged(Option<String>),
}

#[derive(Debug, Clone)]
pub enum XxteaSettingsMessage {
    KeyChanged(Option<String>),
}

#[derive(Debug, Clone)]
pub enum XxteaCfbSettingsMessage {
    KeyChanged(Option<String>),
    IVChanged(Option<String>),
    BlockSizeChanged(Option<String>),
}
