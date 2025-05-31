use std::fmt::Display;

pub mod enigma;
pub mod xxtea;

#[derive(Default, Clone, Copy)]
pub enum Operation {
    #[default]
    Encrypt,
    Decrypt,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum AlgorithmOption {
    #[default]
    Enigma,
    Xxtea,
    XxteaCfb,
}

impl Display for AlgorithmOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlgorithmOption::Enigma => "Enigma",
                AlgorithmOption::Xxtea => "XXTEA",
                AlgorithmOption::XxteaCfb => "XXTEA CFB",
            }
        )
    }
}

pub trait Algorithm {
    fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
    fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
}
