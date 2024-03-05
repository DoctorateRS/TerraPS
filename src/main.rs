mod config;
mod init;
mod routing;
mod utils;

use serde_json::Value;

use config::load_config;
use init::dir_init;
use utils::write_json;

#[tokio::main]
async fn main() {
    dir_init();

    let mut config = load_config();

    config["dummy_data"] = Value::String("new_dummy_data".to_string());

    match write_json("config/config2.json", config) {
        Ok(_) => println!("Config updated"),
        Err(e) => println!("Cannot update config: {}", e),
    };
}
