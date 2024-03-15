use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub(crate) enum Error {}

#[derive(Debug, Error)]
pub(super) enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to serialize/deserialize config file: {0}")]
    SerdeError(#[from] serde_json::Error),
}
