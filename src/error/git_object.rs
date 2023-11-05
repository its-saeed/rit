use std::num::ParseIntError;

use thiserror::Error;

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
