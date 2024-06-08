mod constants;
mod core;
mod game;
mod routes;
mod utils;

use anyhow::Result;

use routes::app;
use utils::{
    server::{get_server_address, Server},
    upgrade,
};

#[tokio::main]
async fn main() -> Result<()> {
    upgrade().await?;
    {
        println!("IN CASE YOU PAID MONEY FOR THIS, YOU'VE BEEN SCAMMED.");
        println!("       THIS IS A FREE AND OPEN SOURCE PROJECT.       ");
        let (server_address, server_port) = get_server_address();
        let server = Server::new(server_address, server_port);
        server.serve(app()).await
    }
}
