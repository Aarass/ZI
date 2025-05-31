use std::path::PathBuf;
use tokio::task::JoinHandle;

pub struct TcpState {
    pub mode: TcpMode,
    //------------------------------------------
    pub file: Option<PathBuf>,
    pub reciever_adress: Option<String>,
    pub reciever_port: Option<u16>,
    pub is_sending: bool,
    //------------------------------------------
    pub dir_to_store_files: Option<PathBuf>,
    pub my_port: Option<u16>,
    pub is_listening: bool,

    pub join_handle: Option<JoinHandle<()>>,
}

#[derive(Default)]
pub enum TcpMode {
    #[default]
    Sending,
    Receiving,
}

impl Default for TcpState {
    fn default() -> Self {
        let mut base = PathBuf::new();
        base.push(r"C:\");
        base.push("Users");
        base.push("Aleksandar");
        base.push("Documents");

        let mut file = base.clone();
        file.push("fsw_1source");
        file.push("New Text Document.txt");

        let mut dir = base.clone();
        dir.push("fsw_2dest");

        Self {
            mode: Default::default(),
            file: Some(file),
            reciever_adress: Some("127.0.0.1".to_owned()),
            reciever_port: Some(80),
            is_sending: Default::default(),
            dir_to_store_files: Some(dir),
            my_port: Some(80),
            is_listening: Default::default(),
            join_handle: Default::default(),
        }
    }
}
