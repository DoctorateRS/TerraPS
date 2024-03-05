mod datapath;
mod init;
mod routing;
mod user;
mod utils;

use datapath::*;
use utils::read_json;

#[tokio::main]
async fn main() {
    let config = read_json(CONFIG_JSON_PATH).unwrap();

    println!("{:?}", config)
}
