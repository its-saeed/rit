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

    #[error(transparent)]
    KvlParseError(#[from] KvlParseError),
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

#[derive(Debug, Error)]
pub enum KvlParseError {
    #[error("Failed to get the next line of the kvl")]
    FailedToGetNextLine,

    #[error("Failed to find the delimiter fo key/value")]
    KeyDelimiterNotFound,
}
