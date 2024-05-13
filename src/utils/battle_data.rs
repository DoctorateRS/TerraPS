use aes::{
    cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit},
    Aes128,
};
use anyhow::Result;
use cbc::Decryptor;
use ring::test::from_hex;
use serde_json::{from_str, json, Value};

use super::crypto::md5::md5_digest;

const DEFAULT_LOGIN_TIME: u32 = 1672502400;
const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";
const CHAR_LIST: [char; 3] = ['\u{8}', '(', ')'];

type Aes128CbcDec = Decryptor<Aes128>;

pub struct BattleDataDecoder {
    login_time: u32,
}

impl Default for BattleDataDecoder {
    fn default() -> Self {
        Self {
            login_time: DEFAULT_LOGIN_TIME,
        }
    }
}

impl BattleDataDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn new_with_login_time(login_time: u32) -> Self {
        Self { login_time }
    }

    pub fn decrypt_battle_data(&self, data: String) -> Result<Value> {
        let (data, iv) = data.split_at(data.len() - 32);
        let src = LOG_TOKEN_KEY.to_string() + &self.login_time.to_string();
        let data = from_hex(data).unwrap();
        let iv = from_hex(iv).unwrap();
        let key = md5_digest(src.as_bytes());
        let aes = Aes128CbcDec::new(key.as_slice().into(), iv.as_slice().into());
        let res = match aes.decrypt_padded_vec_mut::<NoPadding>(&data) {
            Ok(res) => res,
            Err(_) => {
                return Ok(from_str("{}")?);
            }
        };
        let mut res = match String::from_utf8(res) {
            Ok(res) => res,
            Err(_) => {
                return Ok(from_str("{}")?);
            }
        };
        for char in CHAR_LIST {
            res = res.replace(char, "");
        }

        Ok(from_str(&res).unwrap_or(json!({})))
    }
}
