pub mod core;
pub mod game;
pub mod routes;

use anyhow::{Error, Result};
use axum::{serve, Router};
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt::fmt as subscriber_fmt;

use routes::app;

pub struct Server<'s> {
    pub host: &'s String,
    pub port: u16,
}

impl<'s> Server<'s> {
    pub fn new(host: &'s String, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn run(&self) -> Result<()> {
        subscriber_fmt().with_max_level(Level::DEBUG).compact().init();
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;
        match serve(listener, app()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e)),
        }
    }
}
