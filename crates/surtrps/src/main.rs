use std::{env::args, sync::LazyLock};

use anyhow::Result;
use common_utils::{AssetConfig, ServerConfig, UserConfig};
use server::Server;
use utils::update::update;

mod cnst;
mod server;
mod utils;

static USER_CONFIG: LazyLock<UserConfig> = LazyLock::new(|| UserConfig::load().unwrap_or_default());
static SERVER_CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| ServerConfig::load().unwrap_or_default());
static ASSET_CONFIG: LazyLock<AssetConfig> = LazyLock::new(|| AssetConfig::load().unwrap_or_default());

#[tokio::main]
async fn main() -> Result<()> {
    let args = args().collect::<Vec<_>>();

    update().await?;

    let server = Server::new(&SERVER_CONFIG.host, SERVER_CONFIG.port);

    if !(args.contains(&String::from("--update")) || args.contains(&String::from("-u"))) {
        if let Err(e) = server.run().await {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}
