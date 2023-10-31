# Create a GitRepository struct

## [Create a simple GitRepository struct](https://github.com/its-saeed/rit/commit/78697ce7e63d244049158795e0efa6f9f33c22e1)

### Good to know before start

#### Different between `Path` and `PathBuf` in rust

The difference between `Path` and `PathBuf` is roughly the same as the one between \&str and String or &\[] and Vec, ie. Path only holds a reference to the path string data but doesn’t own this data, while PathBuf owns the string data itself. This means that a Path is immutable and can’t be used longer than the actual data (held somewhere else) is available.

The reason why both types exist is to avoid allocations where possible. As most functions take both Path and PathBuf as arguments (by using AsRef for example), this usually doesn’t have a big impact on your code.

A very rough guide for when to use Path vs. PathBuf:

* For return types:
  * If your function gets passed a Path\[Buf] and returns a subpath of it, you can just return a Path (like Path\[Buf].parent())
  * If you create a new path, or combine paths, or anything like that, you need to return a PathBuf.
* For arguments:
  * Take a PathBuf if you need to store it somewhere.
  * Use a Path otherwise.
  * In public interfaces, you usually don’t want to use Path or PathBuf directly, but rather a generic P: AsRef or P: Into. That way the caller can pass in Path, PathBuf, \&str or String.

#### The repository object - Original reference

[Source](https://wyag.thb.lt/#init)

### Implementation

1. Add a new module to `src/lib.rs`

```rust
// src/lib.rs
pub mod repository;
```

2. Add `GitRepository` struct to this module:

```rust
use configparser::ini::Ini;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::git_config::GitConfig;

#[derive(Debug)]
pub struct GitRepository {
    worktree: PathBuf,
    config: GitConfig,
}
```

3. Add two auxiliary functions to `GitConfig`

```rust
impl GitRepository {
    // To return .git directory
    pub fn git_dir(path: &Path) -> PathBuf {
        path.to_owned().join(".git")
    }

    // To return a file path in .git directory
    pub fn repo_path(git_path: &Path, paths: &[&str]) -> PathBuf {
        let mut git_dir = git_path.to_owned();
        for path in paths {
            git_dir.push(path);
        }
        git_dir
    }
}
```

4. Add a few tests to test these two functions:

```rust
#[cfg(test)]
mod tests {
    const PROJECT_DIR: &'static str = "~/home/projects/test";
    use std::path::Path;

    use super::GitRepository;

    #[test]
    fn should_return_correct_git_path() {
        assert_eq!(
            GitRepository::git_dir(&Path::new(&PROJECT_DIR)),
            Path::new("~/home/projects/test/.git")
        );
    }

    #[test]
    fn repo_path_function_should_return_correct_path() {
        let git_path = GitRepository::git_dir(Path::new(PROJECT_DIR));
        assert_eq!(
            GitRepository::repo_path(&git_path, &["config"]),
            Path::new("~/home/projects/test/.git/config")
        );

        assert_eq!(
            GitRepository::repo_path(&git_path, &["another", "file"]),
            Path::new("~/home/projects/test/.git/another/file")
        );
    }
}
```

5. Add `new` function for `GitRepository` struct:

```rust
impl GitRepository {
    pub fn new(path: &Path, _force: bool) -> Result<Self, String> {
        // Get .git path
        let git_path = GitRepository::git_dir(path);

        // Get .git/config file path
        let config_file = GitRepository::repo_path(&git_path, &["config"]);

        // Open and parse config file
        let mut config_file = File::open(config_file).map_err(|e| e.to_string())?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .map_err(|e| e.to_string())?;

        let config: GitConfig = config_string.parse()?;

        // If the version is invalid return an error
        let version = config.repository_format_version()?;
        if version != 0 {
            return Err(format!(
                "Repository format version {} not supported",
                version
            ));
        }

        // Otherwise, return a GitRepository object
        Ok(Self {
            worktree: path.into(),
            config,
        })
    }
}
```
