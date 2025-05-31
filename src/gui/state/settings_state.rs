use crate::algorithms::AlgorithmOption;

use super::args::{EnigmaArgs, XxteaArgs, XxteaCfbArgs};

#[derive(Default, Clone)]
pub struct SettingsState {
    pub algorithm_option: AlgorithmOption,
    pub enigma_args: EnigmaArgs,
    pub xxtea_args: XxteaArgs,
    pub xxtea_cfb_args: XxteaCfbArgs,
}
