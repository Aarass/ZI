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
    Enigma(EnigmaSettingsMesasge),
    Xxtea(XxteaSettingsMesasge),
    XxteaCfb(XxteaCfbSettingsMesasge),
}

#[derive(Debug, Clone)]
pub enum EnigmaSettingsMesasge {
    ReflWiringChanged(Option<String>),
    Rot1WiringChanged(Option<String>),
    Rot1NotchChanged(Option<String>),
    Rot1PositionChanged(Option<String>),
    Rot2WiringChanged(Option<String>),
    Rot2NotchChanged(Option<String>),
    Rot2PositionChanged(Option<String>),
    Rot3WiringChanged(Option<String>),
    Rot3NotchChanged(Option<String>),
    Rot3PositionChanged(Option<String>),
    PlugboardChanged(Option<String>),
}

#[derive(Debug, Clone)]
pub enum XxteaSettingsMesasge {
    KeyChanged(Option<String>),
}

#[derive(Debug, Clone)]
pub enum XxteaCfbSettingsMesasge {
    KeyChanged(Option<String>),
    IVChanged(Option<String>),
    BlockSizeChanged(Option<String>),
}
