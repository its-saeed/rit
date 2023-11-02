use std::num::ParseIntError;

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
pub enum ParseArgumentsError {
    #[error(transparent)]
    ParseObjectTypeError(#[from] ObjectParseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ObjectParseError {
    #[error("Object type is not valid")]
    InvalidObjectType,

    #[error(transparent)]
    InvalidObjectSize(#[from] ParseIntError),

    #[error("Header size differs from the actual read bytes")]
    MismatchedObjectSize,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ObjectCreateError {
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
