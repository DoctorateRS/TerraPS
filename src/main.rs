mod config;
mod routing;

use config::load_config;

#[tokio::main]
async fn main() {
    let config = load_config();
    println!("Hello, world!");
    println!("{:?}", config)
}
