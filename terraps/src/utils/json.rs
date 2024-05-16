use axum::Json;
use serde::Serialize;
use serde_json::{from_slice, json, ser::PrettyFormatter, Serializer, Value};
use std::{
    fs::{read, DirBuilder, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type JSON = Json<Value>;

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
        DirBuilder::new().recursive(true).create(dir).unwrap();
        OpenOptions::new().write(true).create(true).truncate(false).open(path).unwrap()
    } else {
        OpenOptions::new().write(true).truncate(true).open(path).unwrap()
    };
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    file.write_all(&buf).unwrap();
}

pub fn get_keys(value: &Value) -> Vec<String> {
    let mut keys = Vec::new();
    if let Value::Object(map) = value {
        for key in map.keys() {
            keys.push(key.to_string());
        }
    }
    keys
}

pub fn get_values(value: &Value) -> Vec<Value> {
    let mut values = Vec::new();
    if let Value::Object(map) = value {
        for value in map.values() {
            values.push(value.clone());
        }
    }
    values
}

pub fn get_length(value: &Value) -> usize {
    if let Value::Array(array) = value {
        array.len()
    } else {
        0
    }
}
