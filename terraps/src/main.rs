mod constants;
mod core;
mod game;
mod init;
mod routes;
mod utils;

use std::env::args;

use anyhow::Result;

use init::init;

use routes::app;
use utils::{
    server::{get_server_address, Server},
    upgrade,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("TerraPS is starting...");
    init().await?;
    let arg_vec = args().collect::<Vec<_>>();
    let version = env!("CARGO_PKG_VERSION");
    upgrade().await?;
    if !arg_vec.contains(&"--upgrade".to_string()) {
        println!("TerraPS {} is starting...", version);
        let (server_address, server_port) = get_server_address();
        let server = Server::new(server_address, server_port);
        server.serve(app()).await
    } else {
        println!("TerraPS {} has been upgraded!", version);
        Ok(())
    }
}
