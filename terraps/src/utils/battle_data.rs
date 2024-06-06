use aes::{
    cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit},
    Aes128,
};
use cbc::Decryptor;
use rand::{thread_rng, Rng};
use ring::test::from_hex;
use serde_json::{from_str, Value};
use std::io::Write;

use super::{crypto::md5::md5_digest, fs::mkfile};

const DEFAULT_LOGIN_TIME: u32 = 1672502400;
const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";
const FILTER: [char; 3] = ['\u{8}', '(', ')'];

const DUMMY_BATTLE_DATA_TOWER: &str = r#"
{
    "completeState": 0,
    "battleData": {
        "stats": {
            "extraBattleInfo": {}
        }
    }
}
"#;

const DUMMY_BATTLE_DATA_RLV2: &str = r#"
{
    "completeState": 0
}
"#;

const DUMMY_BATTLE_DATA_APRILFOOLS: &str = r#"
{
    "completeState": 0,
    "battleData": {
        "stats": {
            "extraBattleInfo": {}
        }
    }
}
"#;

pub enum FallbackMode {
    Tower,
    Rlv2,
    AprilFools,
}

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

fn rand_hash(len: usize) -> String {
    let mut hash = String::new();
    let mut rng = thread_rng();
    for _ in 0..len {
        let c = rng.gen_range(u8::MIN..u8::MAX);
        hash.push_str(&format!("{:x}", c));
    }
    hash
}

impl BattleDataDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn new_with_login_time(login_time: u32) -> Self {
        Self { login_time }
    }

    pub fn decrypt_battle_data(&self, data: String, mode: FallbackMode) -> Value {
        let fallback = match mode {
            FallbackMode::Tower => from_str(DUMMY_BATTLE_DATA_TOWER).unwrap(),
            FallbackMode::Rlv2 => from_str(DUMMY_BATTLE_DATA_RLV2).unwrap(),
            FallbackMode::AprilFools => from_str(DUMMY_BATTLE_DATA_APRILFOOLS).unwrap(),
        };
        // CREDIT TO ENOKI
        let (data, iv) = data.split_at(data.len() - 32);
        let src = LOG_TOKEN_KEY.to_string() + &self.login_time.to_string();
        let data = from_hex(data).unwrap();
        let iv = from_hex(iv).unwrap();
        let key = md5_digest(src.as_bytes());
        let aes = Aes128CbcDec::new(key.as_slice().into(), iv.as_slice().into());
        let res = match aes.decrypt_padded_vec_mut::<NoPadding>(&data) {
            Ok(res) => res,
            Err(_) => {
                return fallback;
            }
        };
        let mut res = String::from_utf8(res).unwrap();
        for char in FILTER {
            res = res.replace(char, "");
        }
        let res = res.trim();

        let res_val = from_str::<Value>(res);
        match res_val {
            Ok(res) => res,
            Err(_) => {
                let mut dump = mkfile(format!("./dump/{}.txt", rand_hash(8))).unwrap();
                dump.write_all(res.as_bytes()).unwrap();
                fallback
            }
        }
    }
}
