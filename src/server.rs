mod constants;
mod core;
mod debug;
mod game;
mod routes;
mod utils;

use constants::{ascii::TITLE, config::CONFIG_JSON_PATH};
use routes::routes;
use std::io::Error;
use tracing::Level;
use tracing_subscriber::fmt as subscriber_fmt;
use utils::{json::read_json, server::Server};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TRACING
    subscriber_fmt().with_max_level(Level::DEBUG).init();

    // TITLE
    println!(r#"{}"#, TITLE);

    println!("IN CASE YOU PAID MONEY FOR THIS, YOU'VE BEEN SCAMMED.");
    println!("       THIS IS A FREE AND OPEN SOURCE PROJECT.       ");

    // SERVER
    let server_address = get_server_address();
    let server = Server::new(server_address.0, server_address.1);
    server.serve(routes()).await
}

fn get_server_address() -> (String, u64) {
    let config = read_json(CONFIG_JSON_PATH);
    let server_config = &config["server"];
    let host = server_config["host"].as_str().unwrap();
    let port = server_config["port"].as_u64().unwrap();
    (host.to_string(), port)
}
