#[derive(Clone)]
pub struct EnigmaArgs {
    pub refl_wiring: Option<String>,

    pub rot1_wiring: Option<String>,
    pub rot1_notch: Option<String>,
    pub rot1_position: Option<String>,

    pub rot2_wiring: Option<String>,
    pub rot2_notch: Option<String>,
    pub rot2_position: Option<String>,

    pub rot3_wiring: Option<String>,
    pub rot3_notch: Option<String>,
    pub rot3_position: Option<String>,

    pub plugboard: Option<String>,
}

impl Default for EnigmaArgs {
    fn default() -> Self {
        EnigmaArgs {
            refl_wiring: Some("yruhqsldpxngokmiebfzcwvjat".to_owned()),
            rot1_wiring: Some("ekmflgdqvzntowyhxuspaibrcj".to_owned()),
            rot1_notch: Some("8".to_owned()),
            rot1_position: Some("0".to_owned()),
            rot2_wiring: Some("ajdksiruxblhwtmcqgznpyfvoe".to_owned()),
            rot2_notch: Some("8".to_owned()),
            rot2_position: Some("0".to_owned()),
            rot3_wiring: Some("bdfhjlcprtxvznyeiwgakmusqo".to_owned()),
            rot3_notch: Some("0".to_owned()),
            rot3_position: Some("0".to_owned()),
            plugboard: Some("po ml iu kj nh yt gb vf re dc".to_owned()),
        }
    }
}

#[derive(Clone)]
pub struct XxteaArgs {
    pub key: Option<String>,
}

impl Default for XxteaArgs {
    fn default() -> Self {
        Self {
            key: Some("SecureKey".to_owned()),
        }
    }
}

#[derive(Clone)]
pub struct XxteaCfbArgs {
    pub key: Option<String>,
    pub iv: Option<String>,
    pub block_size: Option<String>,
}

impl Default for XxteaCfbArgs {
    fn default() -> Self {
        Self {
            key: Some("SecureKey".to_owned()),
            iv: Some("asdjgasdjgasdjfasdjkhasdf".to_owned()),
            block_size: Some("8".to_owned()),
        }
    }
}
