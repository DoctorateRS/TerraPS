use crate::utils::crypto::base64::decode_bytes;
use anyhow::{Ok, Result};
use serde_json::{from_str, Value};
use zip::{ZipArchive, ZipWriter};

use std::io::{read_to_string, BufReader, Cursor, Write};

use super::crypto::base64::encode_bytes;

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

pub fn encrypt_battle_replay(data: Value) -> Result<String> {
    let buffer = Vec::new();
    let str = data.to_string();
    let mut zip_writer = ZipWriter::new(Cursor::new(buffer));
    zip_writer.start_file("default_entry", Default::default())?;
    zip_writer.write_all(str.as_bytes())?;
    let bytes = zip_writer.finish()?.into_inner();
    Ok(encode_bytes(bytes))
}
