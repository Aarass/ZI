pub mod fsw;
pub mod manual;
pub mod settings;
pub mod tcp;

#[derive(Default)]
pub enum Page {
    Settings,
    #[default]
    Fsw,
    Manual,
    Tcp,
}
