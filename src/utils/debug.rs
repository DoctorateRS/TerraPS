use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use crate::constants::debug::DEBUG;

pub fn debug<T: Debug>(data: T, path: &str) {
    let path = DEBUG.to_owned() + path;
    let file = OpenOptions::new().create(true).truncate(false).write(true).open(path).unwrap();
    let mut file = BufWriter::new(file);
    writeln!(file, "{:?}", data).unwrap();
}
