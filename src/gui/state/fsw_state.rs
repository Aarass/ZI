use notify::Watcher;
use std::path::PathBuf;

use crate::algorithms::Operation;

pub struct FSWState {
    pub from: Option<PathBuf>,
    pub to: Option<PathBuf>,
    pub mode: Operation,
    pub is_on: bool,

    pub watcher: Option<Box<dyn Watcher + Send>>,
}

impl Default for FSWState {
    fn default() -> Self {
        let mut base = PathBuf::new();
        base.push(r"C:\");
        base.push("Users");
        base.push("Aleksandar");
        base.push("Documents");

        let mut from = base.clone();
        from.push("fsw_1source");

        let mut to = base.clone();
        to.push("fsw_2dest");

        Self {
            from: Some(from),
            to: Some(to),
            is_on: Default::default(),
            watcher: Default::default(),
            mode: Default::default(),
        }
    }
}
