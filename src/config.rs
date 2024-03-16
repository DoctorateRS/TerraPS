use crate::errors::ConfigError;
use crate::{constants::CONFIG_JSON_PATH, forms::*};
use std::{
    fs::{read_to_string, File},
    path::Path,
};

impl Config {
    pub fn try_load() -> Result<Self, ConfigError> {
        let content = read_to_string(CONFIG_JSON_PATH).unwrap();
        Ok(serde_json::from_str(&content)?)
    }
    pub fn try_save(&self) -> Result<(), ConfigError> {
        let file = if !Path::new(CONFIG_JSON_PATH).exists() {
            File::create(CONFIG_JSON_PATH)?
        } else {
            File::open(CONFIG_JSON_PATH)?
        };
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_server: true,
            maintenance_msg: "The server is currently under maintenance.".to_string(),
            mode: "cn".to_string(),
            adaptive: true,
            no_proxy: false,
            timeout: 30,
        }
    }
}
