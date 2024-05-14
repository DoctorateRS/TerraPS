use std::io::Error;

use anyhow::Result;
use axum::{serve, Router};
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt as subscriber_fmt;

use crate::constants::config::CONFIG_JSON_PATH;

use super::json::read_json;

pub struct Server {
    pub ip: String,
    pub port: u64,
}

impl Server {
    pub fn new(ip: String, port: u64) -> Server {
        Server { ip, port }
    }
    fn get_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
    fn log_begin(&self) {
        println!("Server started at: {}", self.get_address());
    }
    pub async fn serve(&self, routes: Router) -> Result<(), Error> {
        subscriber_fmt()
            .with_max_level(Level::DEBUG)
            .with_ansi(true)
            .with_file(true)
            .with_line_number(true)
            .compact()
            .init();
        let addr = &self.get_address();
        let listener = TcpListener::bind(addr).await?;
        self.log_begin();
        serve(listener, routes).await
    }
}

pub fn get_server_address() -> (String, u64) {
    let config = read_json(CONFIG_JSON_PATH);
    let server_config = &config["server"];
    let host = server_config["host"].as_str().unwrap();
    let port = server_config["port"].as_u64().unwrap();
    (host.to_string(), port)
}
