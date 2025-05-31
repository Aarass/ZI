use anyhow::anyhow;
use rfd::AsyncFileDialog;
use std::path::Path;
use std::{net::Ipv4Addr, path::PathBuf, str::FromStr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::algorithms::enigma::alg::Enigma;
use crate::algorithms::xxtea::alg::{Xxtea, XxteaCfb};
use crate::algorithms::{Algorithm, AlgorithmOption, Operation};
use crate::gui::state::SettingsState;

pub async fn process_file<T: Algorithm + ?Sized>(
    file: &PathBuf,
    alg: &Box<T>,
    op: Operation,
    dest_dir: &Path,
) -> anyhow::Result<()> {
    let mut file_handle = tokio::fs::OpenOptions::new().read(true).open(&file).await?;
    let file_content = {
        let mut file_buffer = match file_handle.metadata().await {
            Ok(metadata) => Vec::with_capacity(metadata.len().try_into().unwrap()),
            Err(_) => Vec::new(),
        };
        file_handle.read_to_end(&mut file_buffer).await?;
        file_buffer
    };

    let processed_file_content = match op {
        Operation::Encrypt => alg.encrypt(&file_content),
        Operation::Decrypt => alg.decrypt(&file_content),
    }?;

    let new_file_path = get_new_file_path(file, dest_dir, op).await?;
    let mut new_file = tokio::fs::File::create(new_file_path).await?;
    new_file.write_all(&processed_file_content).await?;

    Ok(())
}

pub async fn get_new_file_path(
    file: &Path,
    dest_dir: &Path,
    op: Operation,
) -> anyhow::Result<PathBuf> {
    let file_stem = file.file_stem().unwrap_or_default().to_str().unwrap();
    let extension = file
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    for i in 0..100 {
        let num = format!(" ({})", i);
        let num = if i == 0 { "" } else { &num };

        let name = match op {
            Operation::Encrypt => format!("{}_encrypted{}.{}", file_stem, num, extension),
            Operation::Decrypt => format!("{}_decrypted{}.{}", file_stem, num, extension),
        };

        let new_path = dest_dir.join(name);

        if !tokio::fs::try_exists(&new_path).await? {
            return Ok(new_path);
        }
    }

    Err(anyhow!("Couldn't find available name for the result file"))
}

pub async fn get_new_file_path2(
    filename: &str,
    dest_dir: &Path,
    op: Operation,
) -> anyhow::Result<PathBuf> {
    let parts: Vec<&str> = filename.split(".").collect();

    if parts.len() != 2 {
        return Err(anyhow!("Hellow"));
    }

    let file_stem = parts[0];
    let extension = parts[1];

    println!(
        "get file name 2 - file_stem: {}, extension: {}",
        file_stem, extension
    );

    for i in 0..100 {
        let num = format!(" ({})", i);
        let num = if i == 0 { "" } else { &num };

        let name = match op {
            Operation::Encrypt => format!("{}_encrypted{}.{}", file_stem, num, extension),
            Operation::Decrypt => format!("{}_decrypted{}.{}", file_stem, num, extension),
        };

        let new_path = dest_dir.join(name);

        if !tokio::fs::try_exists(&new_path).await? {
            return Ok(new_path);
        }
    }

    Err(anyhow!("Couldn't find available name for the result file"))
}

pub fn get_algorithm(settings: &SettingsState) -> anyhow::Result<Box<dyn Algorithm + Send + Sync>> {
    match settings.algorithm_option {
        AlgorithmOption::Enigma => Ok(Box::new(Enigma::try_new(&settings.enigma_args)?)),
        AlgorithmOption::Xxtea => Ok(Box::new(Xxtea::try_new(&settings.xxtea_args)?)),
        AlgorithmOption::XxteaCfb => Ok(Box::new(XxteaCfb::try_new(&settings.xxtea_cfb_args)?)),
    }
}

pub fn valid_address(address: &Option<String>) -> bool {
    match address {
        Some(address) => {
            return Ipv4Addr::from_str(address.as_str()).is_ok();
        }
        None => {
            return false;
        }
    };
}

pub fn valid_port(port: &Option<u16>) -> bool {
    if port.is_none() {
        return false;
    }
    true
}

pub async fn get_file_path() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_directory("/")
        .pick_file()
        .await
        .map(|fh| fh.path().to_owned())
}

pub async fn get_dir_path() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_directory("/")
        .pick_folder()
        .await
        .map(|fh| fh.path().to_owned())
}
