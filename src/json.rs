use serde_json::{from_reader, to_writer, Result, Value};
use std::{fs::File, io::BufReader};

pub fn read_json(path: &str) -> Result<Value> {
    let json_reader = BufReader::new(File::open(path).unwrap());
    from_reader(json_reader)
}

pub fn write_json(path: &str, value: &Value) -> Result<()> {
    let file = File::create(path).unwrap();
    to_writer(file, value)
}
