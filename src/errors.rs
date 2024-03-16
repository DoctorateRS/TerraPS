use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub(crate) enum Error {}

#[derive(Debug, Error)]
pub(super) enum ConfigError {
    #[error("IO Errors: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Serde Errors: {0}")]
    SerdeError(#[from] serde_json::Error),
}
