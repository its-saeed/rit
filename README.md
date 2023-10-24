# Let's create a new git in rust!

## 1 - [Create the project structure](https://github.com/its-saeed/rit/commit/fd6fa5295b3b704da2f73b4b4aa87557a5874d0f)
Create a new rust project.
```bash
cargo new rit
```

Create a library file named `lib.rs` and add it as a library crate to `Cargo.toml`:

```toml
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "rit"
```

## 2 - [Add initial dependencies to the project](https://github.com/its-saeed/rit/commit/48fe2c298d9922c64095b1f7e6559bd5249b1a7a)
Add these crates to the project's dependencies:

```toml
[dependencies]
clap = { version = "4.4.6", features = ["cargo"] }
configparser = "3.0.2"
flate2 = "1.0.28"
sha1 = "0.10.6"

```
## 3 - [Start parsing first command, init](https://github.com/its-saeed/rit/commit/65630f9587a8ccd0f498aea46b11ef668dc3155a)
Add parsing arguments basics using `clap` crate. Parse to have a simple CLI realizing the `init` command. No more than just respecting `rit init`

1. Create a new module named `cli`
```rust
// src/lib.rs
pub mod cli;
```

the directory structure should be like:
```
src/
├── cli
│   └── mod.rs
├── lib.rs
└── main.rs
```


2. Start parsing arguments in `src/cli/mod.rs` like:

We're not going to use any crates for errors right now, Let's just return a simple `String` error.


```rust
// 1. Import necessary structs.
use clap::{command, Arg, Command as ClapCommand};

// 2. Add Debug trait to be able to print it out
#[derive(Debug)]
pub enum Command {
    Init,
}

// 3. Implement initial argument parsing.
pub fn parse_args() -> Result<Command, String> {
    let matches = command!()
        .subcommand(ClapCommand::new("init").arg(Arg::new("init")))
        .get_matches();

    match matches.subcommand_matches("init") {
        Some(_subcommand) => Ok(Command::Init),
        None => Err("Failed to parse".to_string()),
    }
}

```
3. `Use` cli structs and functions publicly so we can use them like `rit::parse_args` instead of `rit::cli::parse_args`
```rust
// src/lib.rs

pub use cli::*;
```
4. Use `parse_args` and print the results in main:
```rust
// src/main.rs
use rit::parse_args;

fn main() {
    let command = parse_args().unwrap();
    println!("{:?}", command);
}
```

## 4 - Create a GitConfig struct
A git repository is made of two things: a “work tree”, where the files meant to be in version control live, and a “git directory”, where Git stores its own data. In most cases, the worktree is a regular directory and the git directory is a child directory of the worktree, called .git. [[source]](https://wyag.thb.lt/#init)

In this part we're going to parse `.git/config` file (an INI file) and have a function to return `core.repositoryformatversion`

1. Create a new module named `git_config`. Add this module to `lib.rs`
```rust
// src/lib.rs
pub mod git_config;
```
2. Create a new file in src folder for this module. 
```
src/
├── cli
│   └── mod.rs
├── git_config.rs
├── lib.rs
└── main.rs
```
3. Add `GitConfig` struct to this module: 
```rust
use std::{collections::HashMap, str::FromStr};

use configparser::ini::Ini;

// This is the type of parsed ini file returned by `configparser` crate
type Config = HashMap<String, HashMap<String, Option<String>>>;

#[derive(Debug)]
pub struct GitConfig {
    config: Config,
}
```

4. Implement `FromStr` trait for `GitConfig` so that is possible to parse a string to `GitConfig`:

```rust
impl FromStr for GitConfig {
    type Err = String;

    fn from_str(config_str: &str) -> Result<Self, Self::Err> {
        let mut config = Ini::new();
        let config = config
            .read(config_str.to_string())
            .map_err(|_| "Failed to parse config".to_string())?;

        Ok(Self { config })
    }
}
```

5. Implement `repository_format_version` function for `GitConfig` to return version:
```rust
impl GitConfig {
    pub fn repository_format_version(&self) -> Result<u16, String> {
        // Check if `core` exists, otherwise return error.
        let core = self
            .config
            .get("core")
            .ok_or("Core section doesn't exist".to_string())?;

        // Check if version exists, try to parse it to u16, return error if fails.
        match core
            .get("repositoryformatversion")
            .ok_or("repositoryformatversion not found")?
            .clone()
            .map(|ver| ver.parse::<u16>())
            .transpose()
            .map_err(|e| e.to_string())?
        {
            Some(v) => Ok(v),
            None => return Err("Failed to parse repositoryformatversion".to_string()),
        }
    }
}
```

6. Add a few tests to this module:
```rust
#[cfg(test)]
mod tests {
    use super::GitConfig;

    #[test]
    fn if_config_string_is_valid_repository_format_version_should_return_version() {
        let config_string = r#"
        [core]
            bare = false
            repositoryformatversion = 0
        "#;

        let config: GitConfig = config_string.parse().unwrap();

        assert_eq!(config.repository_format_version().unwrap(), 0);
    }

    #[test]
    fn if_config_string_doesnt_have_version_repository_format_version_function_should_return_error()
    {
        let config_string = r#"
        [core]
            bare = false
        "#;

        let config: GitConfig = config_string.parse().unwrap();
        let version = config.repository_format_version();
        assert!(version.is_err());
    }

    #[test]
    fn if_repository_format_version_is_not_inside_core_function_should_return_error() {
        let config_string = r#"
        [notcore]
            bare = false
            repositoryformatversion = 0
        "#;

        let config: GitConfig = config_string.parse().unwrap();
        let version = config.repository_format_version();
        assert!(version.is_err());
    }
}
```
