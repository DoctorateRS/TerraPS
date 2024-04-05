use tokio::net::TcpListener;

use axum::{serve, Router};
use std::io::Error;
use tracing::Level;
use tracing_subscriber::fmt as subscriber_fmt;

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
    fn log(&self) {
        println!("Server started at: {}", self.get_address());
    }
    pub async fn serve(&self, routes: Router) -> Result<(), Error> {
        subscriber_fmt().with_max_level(Level::DEBUG).init();
        let addr = &self.get_address();
        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => {
                panic!("Failed to bind to address: {}", e);
            }
        };
        self.log();
        serve(listener, routes).await
    }
}
