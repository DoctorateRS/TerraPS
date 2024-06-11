mod constants;
mod core;
mod game;
mod routes;
mod utils;

use anyhow::Result;

use routes::app;
use tracing_log::log::info;
use utils::{
    server::{get_server_address, Server},
    upgrade,
};

#[tokio::main]
async fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    upgrade().await?;
    info!("TerraPS v{} is starting...", version);
    let (server_address, server_port) = get_server_address();
    let server = Server::new(server_address, server_port);
    server.serve(app()).await
}
