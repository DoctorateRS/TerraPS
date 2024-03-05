use serde_json::{from_reader, Result, Value};
use std::{fs::File, io::BufReader};

pub fn load_config() -> Result<Value> {
    let config_reader = BufReader::new(File::open("config/config.json").unwrap());
    let config: Value = from_reader(config_reader).expect("Cannot parse JSON file.");

    Ok(config)
}
