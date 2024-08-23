mod constants;
mod core;
mod game;
mod routes;
mod utils;

use std::env::args;

use anyhow::Result;

use routes::app;
use utils::{
    server::{get_server_address, Server},
    upgrade,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = args().collect::<Vec<_>>();
    let version = env!("CARGO_PKG_VERSION");
    println!("TerraPS is starting...");

    if args.contains(&String::from("--help")) {
        print_help();
        return Ok(());
    }

    upgrade().await?;

    if !args.contains(&String::from("--upgrade")) {
        println!("TerraPS {} is starting...", version);
        let (server_address, server_port) = get_server_address();
        let server = Server::new(server_address, server_port);
        server.serve(app()).await
    } else {
        println!("TerraPS {} has been upgraded!", version);
        Ok(())
    }
}

fn print_help() {
    println!("Help.")
}
