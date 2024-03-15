use crate::errors::ConfigError;
use crate::{constants::CONFIG_JSON_PATH, forms::Config};
use std::fs::{read_to_string, File};

impl Config {
    pub fn try_load() -> Result<Self, ConfigError> {
        let content = read_to_string(CONFIG_JSON_PATH).unwrap();
        Ok(serde_json::from_str(&content)?)
    }
}
