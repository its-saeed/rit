# 10 - [Use a better approach to handle errors]()
## Good to know before start
As you already used `Result`, it's the preferred way to handle errors in rust applications. What if I want to introduce new error types? Unless you don't want to propagate your errors to callers, It's more idiomatic to implement `Error` trait for your new error type. 

An easier approach is to use [thiserror](https://docs.rs/thiserror/latest/thiserror/) crate to create new error types and to have everything ready to handle errors.
We'll use [anyhow](https://docs.rs/thiserror/latest/anyhow) crate as well. So hold this tutorial here and make yourself familiar with these two popular rust crates.

## Implementation

### Add crates to the project
First things first. Add `thiserror` and `anyhow` crates to your project. Change your Cargo.toml or use `cargo add thiserror`

### Add ConfigParseError type and use it
Create a new module `src/error.rs`. And add `ConfigParseError` enum. This enum is used whenever we encounter an error while parsing config file.
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
Let's use this error practically. Here is our previous version of `load_from_file` function in `GitConfig`
```rust
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let mut config_file = File::open(path)
            .map_err(|e| format!("Failed to open config file: {}, {}", path.display(), e))?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .map_err(|e| e.to_string())?;

        Ok(config_string.parse()?)
    }
```

`File::open` returns `std::io::Error` in the case of errors, but we mapped it to an string, because our return type was string. But now we have a better error type `ConfigParseError`
First, let's change the return type to `ConfigParseError`:
```rust
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigParseError> {
```
Now you can easily use this:
```rust
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigParseError> {
        let mut config_file = File::open(path)?;
```
Why? Because we declared `IoError` in `ConfigParseError` and we asked to have `From<std::io::Error` implemented for us. So in the case of std::io::Error, we return `ConfigParseError::IoError`. It's better to add some contexts to errors. Everything that can help us diagonse the error eaiser.

Here is the final Implementation of the function:
```rust
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigParseError> {
        let mut config_file = File::open(path).context("Failed to open config file")?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .context("Failed to read config file")?;

        Ok(config_string.parse()?)
    }
```
You can use `context` function from `anyhow` to add context to the error.
Similarily we can update `repository_format_version` function to use our new error type:
```rust
    pub fn repository_format_version(&self) -> Result<u16, ConfigParseError> {
        let core = self
            .config
            .get("core")
            .ok_or(ConfigParseError::ParseFailed(
                "Core section doesn't exist".to_string(),
            ))?;

        match core
            .get("repositoryformatversion")
            .ok_or(ConfigParseError::ParseFailed(
                "repositoryformatversion not found.".to_string(),
            ))?
            .clone()
            .map(|ver| ver.parse::<u16>())
            .transpose()
            .map_err(|e| ConfigParseError::ParseFailed(e.to_string()))?
        {
            Some(v) => Ok(v),
            None => Err(ConfigParseError::ParseFailed(
                "repositoryformatversion doesn't exist in config".to_string(),
            )),
        }
    }
```

Change `is_repository_format_version_valid` and `from_str` functions yourself to have the new error type. Not that hard.

### Add CreateRepoError error type and use it
Let's add `CreateRepoError` enum to `src/error.rs`:

```rust
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
First we use it in `TryFrom<DirectoryManager>`:
```rust
// src/repository.rs

impl TryFrom<DirectoryManager> for GitRepository {
    // Change Error type to newly created enum
    type Error = CreateRepoError;

    fn try_from(directory_manager: DirectoryManager) -> Result<Self, Self::Error> {
        let config = GitConfig::load_from_file(&directory_manager.config_file)?;

        if !config.is_repository_format_version_valid()? {
            // Instead of returning a string, return appropriate error 
            return Err(CreateRepoError::InvalidRepositoryFormatVersionError);
        }

        Ok(Self {
            config,
            directory_manager,
        })
    }
}
```

Change return type of `load`, `find`, and `create` functions to `Result<Self, CreateRepoError>` and try to make it compile. You can see the final code here LIKE_HERE.

For example, the altered code of `find` is:

```rust
    pub fn find(working_dir: &Path) -> Result<Self, CreateRepoError> {
        match DirectoryManager::is_toplevel_directory(working_dir) {
            true => GitRepository::load(working_dir),
            false => {
                let parent_path = working_dir
                    .parent()
                    // If parent doesn't exist we can't step back further to find the repo toplevel directory.
                    .ok_or(CreateRepoError::NoToplevelFoundError)?;
                GitRepository::find(parent_path)
            }
        }
    }
```

### change the main function to respect the return type of `GitRepository::create`
Here is the previous version of the main. What happens if create function fails? Nothing! The app panics.
```rust
fn main() {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => GitRepository::create(path).unwrap(),
    };
}
```
Let's make it more professional.

```rust
use anyhow::{Ok, Result};
use rit::{parse_args, repository::GitRepository, Command};

fn main() -> Result<()> {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => GitRepository::create(path)?,
    };

    Ok(())
}
```
Now try to break `init` command:

```bash
$ touch /tmp/prj-dir
$ cargo run -- init /tmp/prj-dir
```
You must see `Error: Provided toplevel is not a directory.`

Please take a look at the commit<LINK_HERE> changes. I also added a tiny error type for argument parsing errors.
