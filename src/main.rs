mod constants;
mod core;
mod game;
mod routes;
mod utils;

use anyhow::Result;
use routes::routes;
use utils::{
    server::{get_server_address, Server},
    upgrade,
};

#[tokio::main]
async fn main() -> Result<()> {
    // UPGRADE
    upgrade().await?;

    // TITLE
    println!("IN CASE YOU PAID MONEY FOR THIS, YOU'VE BEEN SCAMMED.");
    println!("       THIS IS A FREE AND OPEN SOURCE PROJECT.       ");

    // SERVER
    let (server_address, server_port) = get_server_address();
    let server = Server::new(server_address, server_port);
    Ok(server.serve(routes()).await?)
}
