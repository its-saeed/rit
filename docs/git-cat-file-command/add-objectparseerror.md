# Add ObjectParseError

In [create-a-git-object-module.md](create-a-git-object-module.md "mention") we said that `read` function to read objects has this signature:

```rust
pub fn read(repo: &GitRepository, sha: String) -> Result<Box<dyn GitObject>, ObjectParseError> {
```

We covered the first part of the return type in [add-blobobject.md](add-blobobject.md "mention"), now we need to add a new error type, `ObjectParseError:`

```rust
// src/error.rs

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
```

