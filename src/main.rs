mod config;
mod constants;
mod init;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    init::init().await;
}
