mod constants;
mod core;
mod game;
mod routes;
mod utils;

use anyhow::Result;
use constants::ascii::TITLE;
use routes::routes;
use utils::{
    fmt::{ccv2_fmt, cfg_fmt, excel_fmt},
    server::{get_server_address, Server},
    update::{excel_update, update_config},
};

#[tokio::main]
async fn main() -> Result<()> {
    update_config().await?;
    excel_update().await?;
    excel_fmt()?;
    cfg_fmt()?;
    ccv2_fmt()?;

    // TITLE
    println!(r#"{}"#, TITLE);
    println!("IN CASE YOU PAID MONEY FOR THIS, YOU'VE BEEN SCAMMED.");
    println!("       THIS IS A FREE AND OPEN SOURCE PROJECT.       ");

    // SERVER

    let (server_address, server_port) = get_server_address();
    let server = Server::new(server_address, server_port);
    Ok(server.serve(routes()).await?)
}
