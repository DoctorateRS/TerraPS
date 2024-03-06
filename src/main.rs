mod datapath;
mod init;
mod prod_config;
mod routing;
mod update;
mod user;
mod utils;

use datapath::*;
use init::dir_init;
use prod_config::*;
use utils::read_json;

#[tokio::main]
async fn main() {
    dir_init();
    let config = read_json(CONFIG_JSON_PATH).unwrap();
}
