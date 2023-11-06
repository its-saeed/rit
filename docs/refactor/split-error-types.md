# Split error types

Our error module is getting bigger and bigger. Let's create a new submodule for each:

Create a new file, mod.rs, and add submodules:

{% code title="src/error/mod.rs" lineNumbers="true" %}
```rust
pub mod cli;
pub mod git_config;
pub mod git_object;
pub mod repository;

pub use cli::ParseArgumentsError;
pub use git_config::ConfigParseError;
pub use git_object::*;
pub use repository::CreateRepoError;
```
{% endcode %}

Move each error type to related file:\


{% code title="src/error/cli.rs" lineNumbers="true" %}
```rust
use thiserror::Error;

use super::ObjectParseError;

#[derive(Debug, Error)]
pub enum ParseArgumentsError {
    #[error(transparent)]
    ParseObjectTypeError(#[from] ObjectParseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

```
{% endcode %}

{% code title="src/error/git_config.rs" lineNumbers="true" %}
```rust
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

```
{% endcode %}

{% code title="src/error/git_object.rs" lineNumbers="true" %}
```rust
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

```
{% endcode %}

{% code title="src/error/repository.rs" lineNumbers="true" %}
```rust
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

```
{% endcode %}

Don't forget to remove `src/error.rs` file. We no longer need it.
