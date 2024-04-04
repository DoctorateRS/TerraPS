mod constants;
mod core;
mod crypto;
mod game;
mod routes;
mod utils;

use axum::serve;
use tokio::net::TcpListener as Listener;
use tracing::{info, Level};
use tracing_subscriber::fmt as subscriber_fmt;
use utils::json_utils::read_json;

#[tokio::main]
async fn main() {
    // TRACING
    subscriber_fmt().with_max_level(Level::DEBUG).init();

    // SERVER
    let server_address = &get_server_address();
    let listener = match Listener::bind(server_address).await {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind to address: {}", e);
        }
    };
    info!("Server started at: {}", server_address);
    match serve(listener, routes::routes()).await {
        Ok(_) => (),
        Err(e) => {
            panic!("Failed to start server: {}", e);
        }
    };
}

fn get_server_address() -> String {
    let config = read_json(constants::config::CONFIG_JSON_PATH);
    let server_config = &config["server"];
    let host = server_config["host"].as_str().unwrap();
    let port = server_config["port"].as_u64().unwrap();
    format!("{}:{}", host, port)
}
