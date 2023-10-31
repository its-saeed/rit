# Implement GitRepository::create

### Good to know before start

To create a new repository, we start with a directory (which we create if doesn’t already exist) and create the git directory inside (which must not exist already, or be empty). That directory is called .git (the leading period makes it “hidden” on Unix systems), and contains:

* `git/objects/` : the object store, which we’ll introduce in the next section.
* `git/refs/` the reference store, which we’ll discuss a bit later. It contains two subdirectories, `heads` and `tags`.
* `git/HEAD`, a reference to the current HEAD (more on that later!)
* `git/config`, the repository’s configuration file.
* `git/description`, holds a free-form description of this repository’s contents, for humans, and is rarely used.

### Implementation

Implement `Default` trait for GitConfig:

```rust
// src/git_config.rs

impl GitConfig {
    //...
    pub fn default_str() -> &'static str {
        r#"[core]
            bare = false
            repositoryformatversion = 0
            filemode = false"#
    }
    //...
}

impl Default for GitConfig {
    fn default() -> Self {
        GitConfig::default_str().parse().unwrap()
    }
}

```

Add a function to `DirectoryManager` to create initial directory tree:

```rust
// src/directory_manager.rs
impl DirectoryManager {
    // ...
    pub fn create_directory_tree(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.work_tree)?;
        fs::create_dir_all(&self.dot_git_path)?;
        fs::create_dir_all(self.dot_git_path.join("branches"))?;
        fs::create_dir_all(self.dot_git_path.join("objects"))?;
        fs::create_dir_all(self.dot_git_path.join("refs").join("tags"))?;
        fs::create_dir_all(self.dot_git_path.join("refs").join("heads"))?;
        Ok(())
    }
    // ...
}
```

and let's keep the common paths in the `DirectoryManager`:

```rust
#[derive(Debug)]
pub struct DirectoryManager {
    //...
    pub config_file: PathBuf,
    pub description_file: PathBuf,
    pub head_file: PathBuf,
}

impl DirectoryManager {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        let base_path: PathBuf = base_path.into();
        let dot_git_path = base_path.join(".git");

        Self {
            work_tree: base_path,
            config_file: dot_git_path.join("config"),
            description_file: dot_git_path.join("description"),
            head_file: dot_git_path.join("HEAD"),
            dot_git_path,
        }
    }
}

```

pay attention that `config_file` function is removed. Add an auxiliary function to `DirectoryManager` to check if `.git` is empty:

```rust
    pub fn is_dot_git_empty(&self) -> Result<bool, std::io::Error> {
        Ok(!self.dot_git_path.exists() || self.dot_git_path.read_dir()?.next().is_none())
    }
```

Update tests:

```rust
// src/directory_manager.rs

    #[test]
    fn should_return_correct_file_paths() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.config_file,
            Path::new("~/home/projects/test/.git/config")
        );
        assert_eq!(
            dir_manager.description_file,
            Path::new("~/home/projects/test/.git/description")
        );
        assert_eq!(
            dir_manager.head_file,
            Path::new("~/home/projects/test/.git/HEAD")
        );
    }
```

Now let's implement `GitRepository::create` function:

```rust
// src/repository.rs

    pub fn create(base_path: &Path) -> Result<Self, String> {
        let directory_manager = DirectoryManager::new(base_path);

        // First, check if the work tree exists and if it does, is it a directory?
        if directory_manager.work_tree.exists() && !directory_manager.work_tree.is_dir() {
            return Err(format!("{} is not a directory!", base_path.display()));
        }

        // Then check if .git directory is empty?
        if !directory_manager
            .is_dot_git_empty()
            .map_err(|e| e.to_string())?
        {
            return Err(format!(
                "{} is not empty!",
                directory_manager.dot_git_path.display()
            ));
        }

        // Create the initial directory tree, .git, .git/refs, .git/objects, etc.
        directory_manager
            .create_directory_tree()
            .map_err(|e| e.to_string())?;

        // Write initial contents of .git/description
        std::fs::write(
            &directory_manager.description_file,
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .map_err(|e| e.to_string())?;

        // Write initial contents of .git/HEAD
        std::fs::write(&directory_manager.head_file, "ref: refs/heads/master\n")
            .map_err(|e| e.to_string())?;

        // Write initial contents of .git/config
        std::fs::write(&directory_manager.config_file, GitConfig::default_str())
            .map_err(|e| e.to_string())?;

        Ok(Self {
            directory_manager,
            config: GitConfig::default(),
        })
    }
```
