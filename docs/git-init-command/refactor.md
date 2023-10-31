# Refactor!

## [Refactor!](https://github.com/its-saeed/rit/commit/98dc7d2a5c5ec76f7406f0215d5866284d3713a5)

In the original version (written in Python), based on `force` variable `__init__` of `GitRepository` maybe used either for loading a repository or creating a new one. I don't like that approach. I do believe in the separation of concerns.

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

Currently `GitRepository` is doing too much, doing (at least, at this time) more than one job. Managing the repository and dealing with directories at a low level. So let's moe directory management to a new module named `DirectoryManager`:

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
