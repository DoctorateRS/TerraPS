mod config;
mod routing;

use config::load_config;

#[tokio::main]
async fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => panic!("Cannot load config: {}", e),
    };
}
