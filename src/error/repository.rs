use thiserror::Error;

use super::ConfigParseError;

#[derive(Debug, Error)]
pub enum CreateRepoError {
    #[error("Format version is not valid")]
    InvalidRepositoryFormatVersionError,

    #[error("No git toplevel found in current directory/any of parents")]
    NoToplevelFoundError,

    #[error("Provided toplevel is not a directory.")]
    TopLevelIsNotDirectory,

    #[error("Provided toplevel is not empty.")]
    TopLevelIsNotEmpty,

    #[error(transparent)]
    ConfigError(#[from] ConfigParseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ResolveRefError {
    #[error("Relative path {0} is not a file")]
    RelativePathIsNotAFile(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
