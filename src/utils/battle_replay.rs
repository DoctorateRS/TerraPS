use crate::utils::crypto::base64::decode_bytes;
use anyhow::{Ok, Result};
use serde_json::{from_str, to_string, Value};
use zip::ZipArchive;

use std::io::{read_to_string, BufReader, Cursor};

pub fn decrypt_battle_replay(data: String) -> Result<Value> {
    let decoded_data = decode_bytes(data)?;
    let data = Cursor::new(decoded_data);
    let mut zip_file = ZipArchive::new(data)?;
    let dumped_files = zip_file.by_name("default_entry")?;
    let zip_file = BufReader::new(dumped_files);
    let str = read_to_string(zip_file)?;
    let res = from_str(&str)?;

    Ok(res)
}

// pub fn encrypt_battle_replay(data: Value) -> Result<String> {
//     let str = to_string(&data)?;
// }
