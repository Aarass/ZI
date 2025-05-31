use std::path::PathBuf;

#[derive(Default)]
pub struct ManualState {
    pub from: Option<PathBuf>,
    pub to: Option<PathBuf>,
    pub is_doing_work: bool,
}
