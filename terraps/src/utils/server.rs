use std::str::from_utf8;

use anyhow::Result;
use axum::{serve, Router};
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt::fmt as subfmt;

use crate::constants::config::CONFIG_JSON_PATH;

use super::{crypto::base64::decrypt, time::Time};
use common_utils::read_json;

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
    fn log_something(&self) {
        let binding = decrypt(b"SU4gQ0FTRSBZT1UgUEFJRCBNT05FWSBGT1IgVEhJUywgWU9VJ1ZFIEJFRU4gU0NBTU1FRC4=").unwrap();
        let str = from_utf8(binding.as_slice()).unwrap();
        println!("{}", str);
        let binding = decrypt(b"ICAgICAgIFRISVMgSVMgQSBGUkVFIEFORCBPUEVOIFNPVVJDRSBQUk9KRUNULiAgICAgICA=").unwrap();
        let str = from_utf8(binding.as_slice()).unwrap();
        println!("{}", str);
    }
    fn log_begin(&self) {
        println!("Server started at: {}", self.get_address());
    }
    pub async fn serve(&self, routes: Router) -> Result<()> {
        subfmt()
            .with_max_level(Level::DEBUG)
            .with_timer(Time)
            .with_file(false)
            .with_line_number(false)
            .compact()
            .init();
        let addr = &self.get_address();
        let listener = TcpListener::bind(addr).await?;
        self.log_something();
        self.log_begin();
        match serve(listener, routes).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

pub fn get_server_address() -> (String, u64) {
    let config = read_json(CONFIG_JSON_PATH);
    let host = config["server"]["host"].as_str().unwrap_or("127.0.0.1");
    let port = config["server"]["port"].as_u64().unwrap_or(8443);
    (host.to_owned(), port)
}
