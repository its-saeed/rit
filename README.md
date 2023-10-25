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

## 4 - [Create a GitConfig struct](https://github.com/its-saeed/rit/commit/c9b11d78872f12b5ea3171d04390a2125dc60a07)
### Good to know before start
A git repository is made of two things: a “work tree”, where the files meant to be in version control live, and a “git directory”, where Git stores its own data. In most cases, the worktree is a regular directory and the git directory is a child directory of the worktree, called .git. [[source]](https://wyag.thb.lt/#init)

### Implementation
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



## 5 - [Create a simple GitRepository struct](https://github.com/its-saeed/rit/commit/78697ce7e63d244049158795e0efa6f9f33c22e1)
### Good to know before start
#### Different between `Path` and `PathBuf` in rust

The difference between `Path` and `PathBuf` is roughly the same as the one between &str and String or &[] and Vec, ie. Path only holds a reference to the path string data but doesn’t own this data, while PathBuf owns the string data itself. This means that a Path is immutable and can’t be used longer than the actual data (held somewhere else) is available.

The reason why both types exists is to avoid allocations where possible. As most functions take both Path and PathBuf as arguments (by using AsRef<Path> for example), this usually doesn’t have a big impact on your code.

A very rough guide for when to use Path vs. PathBuf:

* For return types:
  * If your function gets passed a Path[Buf] and returns a subpath of it, you can just return a Path (like Path[Buf].parent())
  * If you create a new path, or combine paths or anything like that, you need to return a PathBuf.
* For arguments:
  * Take a PathBuf if you need to store it somewhere.
  * Use a Path otherwise.
  * In public interfaces, you usually don’t want to use Path or PathBuf directly, but rather a generic P: AsRef<Path> or P: Into<PathBuf>. That way the caller can pass in Path, PathBuf, &str or String.

#### The repository object - Original reference
[Source](https://wyag.thb.lt/#init)

#### Implementation

1. Add new module to `src/lib.rs`
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

        // If version is invalid return an error
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

## 6 - Refactor!
In the [original version]() (written in python), based on `force` variable `__init__` of `GitRepository` may be used either for loading a repository or creating a new one. I don't like that approach. I do believe in separation of concerns.
```python
    def __init__(self, path, force=False):
        self.worktree = path
        self.gitdir = os.path.join(path, ".git")

        if not (force or os.path.isdir(self.gitdir)):
            raise Exception("Not a Git repository %s" % path)

        # Read configuration file in .git/config
        self.conf = configparser.ConfigParser()
        cf = repo_file(self, "config")

        if cf and os.path.exists(cf):
            self.conf.read([cf])
        elif not force:
            raise Exception("Configuration file missing")

        if not force:
            vers = int(self.conf.get("core", "repositoryformatversion"))
            if vers != 0:
                raise Exception("Unsupported repositoryformatversion %s" % vers)
```
Currently `GitRepository` is doing too much, doing (at least, at this time) more than one job. Managing the repository and dealing with directories at low level. So let's moe directory management to a new module named `DirectoryManager`:

```rust
// src/lib.rs
pub mod directory_manager;

pub use directory_manager::DirectoryManager;
```

Add `DirectoryManager` struct in this new module:
```rust
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DirectoryManager {
    pub work_tree: PathBuf, // Where we add our project's files
    pub dot_git_path: PathBuf, // Where git stores its local files
}
```

Add an `impl` block for this struct: 
```rust
impl DirectoryManager {
    // To create a new object
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        let base_path: PathBuf = base_path.into();
        Self {
            dot_git_path: base_path.join(".git"),
            work_tree: base_path,
        }
    }

    // To return config file path. Instead of having a raw function like rep_path, let's have a more meaningful function
    pub fn config_file(&self) -> PathBuf {
        self.dot_git_path.join("config")
    }
}
```

Move tests from `repository.rs` to this module:
```rust
#[cfg(test)]
mod tests {
    const PROJECT_DIR: &'static str = "~/home/projects/test";
    use std::path::Path;

    use crate::DirectoryManager;

    #[test]
    fn should_return_correct_git_path() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.dot_git_path,
            Path::new("~/home/projects/test/.git")
        );
    }

    #[test]
    fn should_return_correct_config_file_path() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.config_file(),
            Path::new("~/home/projects/test/.git/config")
        );
    }
}

```

Change `Repository` to have a `DirectoryManager`

```rust
#[derive(Debug)]
pub struct GitRepository {
    config: GitConfig,
    directory_manager: DirectoryManager,
}
```

Instead of having a `Repository::new` function that does magic with a `force` flag (What does force mean? force to what?) let's have two separate functions `load` and `create`. Names explain the goal. `load` is used to load an existing repository, `create` creates an new one.
```rust
impl GitRepository {
    /// Load an existing repository.
    pub fn load(base_path: &Path) -> Result<Self, String> {
        let directory_manager = DirectoryManager::new(base_path);
        let config_file = directory_manager.config_file();

        let mut config_file = File::open(config_file).map_err(|e| e.to_string())?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .map_err(|e| e.to_string())?;

        let config: GitConfig = config_string.parse()?;

        let version = config.repository_format_version()?;
        if version != 0 {
            return Err(format!(
                "Repository format version {} not supported",
                version
            ));
        }

        Ok(Self {
            config,
            directory_manager,
        })
    }

    /// Create a new repository
    pub fn create(_base_path: &Path) -> Self {
        todo!()
    }
}
```