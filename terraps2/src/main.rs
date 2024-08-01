use std::sync::LazyLock;

use common_utils::UserConfig;

mod cnst;
mod models;
mod server;
mod utils;

static USER_CONFIG: LazyLock<UserConfig> = LazyLock::new(|| UserConfig::load().unwrap_or_default());

#[tokio::main]
async fn main() {}
