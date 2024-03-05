mod config;
mod routing;
mod utils;

use serde_json::{to_string_pretty, Value};

use config::load_config;
use utils::write_json;

#[tokio::main]
async fn main() {
    let mut config = load_config();

    config["dummy_data"] = Value::String("new_dummy_data".to_string());

    match write_json("config/config2.json", config) {
        Ok(_) => println!("Config updated"),
        Err(e) => println!("Cannot update config: {}", e),
    };
}
