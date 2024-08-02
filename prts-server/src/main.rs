use std::sync::LazyLock;

use common_utils::{AssetConfig, ServerConfig, UserConfig};
use server::Server;

mod cnst;
mod models;
mod server;
mod utils;

static USER_CONFIG: LazyLock<UserConfig> = LazyLock::new(|| UserConfig::load().unwrap_or_default());
static SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| ServerConfig::load().unwrap_or_default());
static ASSET_CONFIG: LazyLock<AssetConfig> = LazyLock::new(|| AssetConfig::load().unwrap_or_default());

#[tokio::main]
async fn main() {
    let server = Server::new(&SERVER_CONFIG.host, SERVER_CONFIG.port as u16);

    server.run().await.unwrap();
}
