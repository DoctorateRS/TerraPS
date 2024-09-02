pub mod core;
pub mod game;
pub mod routes;

use anyhow::{Error, Result};
use axum::serve;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt::fmt as subscriber_fmt;

use routes::app;

pub struct Server<'server> {
    pub host: &'server String,
    pub port: i64,
}

impl<'server> Server<'server> {
    pub fn new(host: &'server String, port: i64) -> Self {
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
