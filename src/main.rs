mod assets;
mod crisis;
mod datapath;
mod init;
mod prod_config;
mod routing;
mod user;
mod utils;

use init::init;

#[tokio::main]
async fn main() {
    init().await;
}
