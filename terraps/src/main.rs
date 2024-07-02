mod constants;
mod core;
mod game;
mod routes;
mod utils;

use std::env::args;

use anyhow::Result;

use routes::app;
use tracing::info;
use utils::{
    server::{get_server_address, Server},
    upgrade,
};

#[tokio::main]
async fn main() -> Result<()> {
    let arg_vec = args().collect::<Vec<_>>();
    let version = env!("CARGO_PKG_VERSION");
    upgrade().await?;
    if !arg_vec.contains(&"--upgrade".to_string()) {
        info!("TerraPS {} is starting...", version);
        let (server_address, server_port) = get_server_address();
        let server = Server::new(server_address, server_port);
        server.serve(app()).await
    } else {
        info!("TerraPS {} has been upgraded!", version);
        Ok(())
    }
}
