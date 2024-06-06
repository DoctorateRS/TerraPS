use std::{
    fs::{read, DirBuilder, OpenOptions},
    io::Write,
    path::PathBuf,
};

use serde::Serialize;
use serde_json::{from_slice, json, ser::PrettyFormatter, Serializer, Value};

mod fs;
mod json;

pub fn read_json(path: &str) -> Value {
    let val = match read(path) {
        Ok(file) => file,
        Err(_) => {
            let dir = PathBuf::from(path);
            let dir = dir.parent().unwrap();
            DirBuilder::new().recursive(true).create(dir).unwrap();
            return json!({});
        }
    };
    match from_slice(&val) {
        Ok(value) => value,
        Err(_) => json!({}),
    }
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
