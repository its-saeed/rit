# Create a GitConfig struct

## [Create a GitConfig struct](https://github.com/its-saeed/rit/commit/c9b11d78872f12b5ea3171d04390a2125dc60a07)

### Good to know before start

A git repository is made of two things: a “work tree”, where the files are meant to be in version control live, and a “git directory”, where Git stores its own data. In most cases, the worktree is a regular directory and the git directory is a child directory of the worktree, called .git. [\[source\]](https://wyag.thb.lt/#init)

### Implementation

In this part, we're going to parse `.git/config` file (an INI file) and have a function to return `core.repositoryformatversion`

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
