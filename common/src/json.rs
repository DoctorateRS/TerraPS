use std::{
    fs::{read, write},
    path::Path,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, from_value, json, ser::PrettyFormatter, Serializer, Value};

fn _read_json_inner<P: AsRef<Path>>(path: P) -> Result<Value> {
    let val = read(path)?;
    Ok(from_slice(&val)?)
}

pub fn read_json<P: AsRef<Path>>(path: P) -> Value {
    _read_json_inner(path).unwrap_or(json!({}))
}

#[allow(dead_code)]
pub fn read_json_to_struct<P: AsRef<Path>, T: for<'de> Deserialize<'de>>(path: P) -> Result<T> {
    let val = _read_json_inner(path)?;
    Ok(from_value(val)?)
}

pub fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: T) -> Result<()> {
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser)?;
    Ok(write(path, buf)?)
}
