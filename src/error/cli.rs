use thiserror::Error;

use super::ObjectParseError;

#[derive(Debug, Error)]
pub enum ParseArgumentsError {
    #[error(transparent)]
    ParseObjectTypeError(#[from] ObjectParseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
