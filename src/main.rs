mod config;
mod datapath;
mod init;
mod routing;
mod user;
mod utils;

use datapath::*;
use init::dir_init;
use utils::read_json;

#[tokio::main]
async fn main() {
    dir_init();
    let config = read_json(CONFIG_JSON_PATH).unwrap();

    println!("{:?}", config)
}
