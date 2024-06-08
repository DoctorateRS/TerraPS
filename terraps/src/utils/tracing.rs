use ansi_term::enable_ansi_support;
use anyhow::Result;
use env_logger::{init_from_env, Env};
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, EnvFilter, Registry};

pub fn init_tracing() -> Result<()> {
    #[cfg(target_os = "windows")]
    let _ = enable_ansi_support().unwrap_err();
    init_from_env(Env::new().default_filter_or("info"));

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::try_new("info").unwrap());

    set_global_default(Registry::default().with(filter).with(Layer::default()))?;

    Ok(())
}
