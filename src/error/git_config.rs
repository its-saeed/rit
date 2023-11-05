use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigParseError {
    #[error("Failed to parse config file. {0}")]
    ParseFailed(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
