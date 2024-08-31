use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};

pub fn encrypt<T: AsRef<[u8]>>(data: T) -> String {
    STANDARD.encode(data)
}

pub fn decrypt<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>> {
    Ok(STANDARD.decode(data)?)
}
