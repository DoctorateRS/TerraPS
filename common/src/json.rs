use std::{
    fs::{read, DirBuilder, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, from_value, json, ser::PrettyFormatter, Serializer, Value};

fn _read_json_inner<P: AsRef<Path>>(path: P) -> Result<Value> {
    let val = read(path)?;
    Ok(from_slice(&val)?)
}

pub fn read_json(path: &str) -> Value {
    _read_json_inner(path).unwrap_or(json!({}))
}

#[allow(dead_code)]
pub fn read_json_to_struct<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T> {
    let val = _read_json_inner(path)?;
    Ok(from_value(val)?)
}

pub fn write_json<T: Serialize>(path: &str, value: T) {
    let mut file = if !PathBuf::from(path).exists() {
        let dir = PathBuf::from(path);
        let dir = dir.parent().unwrap();
        DirBuilder::new().recursive(true).create(dir).unwrap_or(());
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap()
    };
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    file.write_all(&buf).unwrap();
}
