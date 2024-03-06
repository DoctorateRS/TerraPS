mod assets;
mod crisis;
mod datapath;
mod init;
mod prod_config;
mod routing;
mod user;
mod utils;

use init::init;
use prod_config::*;

#[tokio::main]
async fn main() {
    init().await;
    println!("{:#?}", prod_network_config().await);
}
