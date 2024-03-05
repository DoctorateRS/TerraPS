use serde_json::Value;

use crate::json::read_json;

pub fn load_config() -> Value {
    match read_json("config/config.json") {
        Ok(config) => config,
        Err(e) => panic!("Cannot load config: {}", e),
    }
}
