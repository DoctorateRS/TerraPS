mod app;
mod config;
mod data;
mod routing;

use config::load_config;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => panic!("Cannot load config: {}", e),
    };
    println!("Config: {}", config);

    let x = [("method", "GET"), ("path", "/")];
}
