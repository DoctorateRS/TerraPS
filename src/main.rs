mod config;
mod routing;
mod utils;

use config::load_config;

#[tokio::main]
async fn main() {
    let config = load_config();

    println!("{}", config);
}
