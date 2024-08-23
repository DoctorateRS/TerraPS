use aes::{
    cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit},
    Aes128,
};
use anyhow::{anyhow, Result};
use cbc::Decryptor;
use ring::test::from_hex;
use serde::de::DeserializeOwned;
use serde_json::from_str;

use super::md5::md5_digest;

const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";

#[derive(Debug, Clone, Copy)]
pub struct BattleDataDecryptor {
    login_time: u64,
}

impl BattleDataDecryptor {
    pub fn new(login_time: Option<u64>) -> Self {
        Self {
            login_time: if let Some(time) = login_time { time } else { 1672502400 },
        }
    }

    pub fn decrypt_battle_data<D: DeserializeOwned>(&self, data: String) -> Result<D> {
        let (data, iv) = data.split_at(data.len() - 32);
        let src = format!("{}{}", LOG_TOKEN_KEY, self.login_time);

        let data = match from_hex(data) {
            Ok(data) => data,
            Err(e) => return Err(anyhow!(e)),
        };

        let iv = match from_hex(iv) {
            Ok(iv) => iv,
            Err(e) => return Err(anyhow!(e)),
        };

        let key = md5_digest(src.as_bytes());

        let aes = Decryptor::<Aes128>::new(key.as_slice().into(), iv.as_slice().into());

        let data = aes.decrypt_padded_vec_mut::<NoPadding>(&data)?;
        let data = String::from_utf8(data)?;

        let data = from_str(&data)?;

        Ok(data)
    }
}
