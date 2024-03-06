mod datapath;
mod init;
mod prod_config;
mod routing;
mod user;
mod utils;

use datapath::*;
use init::init;
use utils::read_json;

#[tokio::main]
async fn main() {
    init();
    let config = read_json(CONFIG_JSON_PATH).unwrap();
}
