# 10 - Add `find` function to GitRepository
While we’re implementing repositories, we’re going to need a function to find the root of the current repository. We’ll use it a lot, since almost all Git functions work on an existing repository (except init, of course!). Sometimes that root is the current directory, but it may also be a parent: your repository’s root may be in ~/Documents/MyProject, but you may currently be working in ~/Documents/MyProject/src/tui/frames/mainview/. The `GitRepository::find` function we’ll now create will look for that root, starting at the current directory and recursing back to /. To identify a path as a repository, it will check for the presence of a .git directory.

Let's start by adding an auxiliary function to `DirectoryManager` to see if a given directory is top level directory of a repository.
```rust
    pub fn is_toplevel_directory(path: &Path) -> bool {
        path.exists() && path.join(".git").is_dir() && path.join(".git/config").is_file()
    }
```
I added a few integration tests to `tests/directory_manager.rs` to verify that `is_toplevel_directory` works fine. You can see them here<ADD_A_LINK>

Throughout the development of any projects, you need to refactor regularly, otherwise you'll end of with a messy code. In the current code, `GitRepository` is responsible for opening and parsing the config file which is not nice. I'm going to move it to `GitConfig` itself:

```rust
impl GitConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let mut config_file = File::open(path)
            .map_err(|e| format!("Failed to open config file: {}, {}", path.display(), e))?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .map_err(|e| e.to_string())?;

        Ok(config_string.parse()?)
    }
    //...

    pub fn is_repository_format_version_valid(&self) -> Result<bool, String> {
        Ok(self.repository_format_version()? == 0)
    }
}

```
As you can see I also created a tiny function to validate format version. Let's use these two functions.

In some circumstances we have a `DirectoryManager` and we want to create a `GitRepository`, In idiomatic rust we can implement `TryFrom<DirectoryManager>` for `GitRepository` so that we can convert a `DirectoryManager` object to a `GitRepository` object. Why didn't we implement `From`? Because it can fails.
```rust
// src/repository.rs

impl TryFrom<DirectoryManager> for GitRepository {
    type Error = String;

    fn try_from(directory_manager: DirectoryManager) -> Result<Self, Self::Error> {
        let config = GitConfig::load_from_file(&directory_manager.config_file)?;

        if !config.is_repository_format_version_valid()? {
            return Err("Repository format version not supported".to_string());
        }

        Ok(Self {
            config,
            directory_manager,
        })
    }
}
```
Now we can refactor `GitRepository::load` function to this clean one:

```rust
// src/repository.rs

    pub fn load<T: Into<PathBuf>>(base_path: T) -> Result<Self, String> {
        GitRepository::try_from(DirectoryManager::new(base_path))
    }

```

And finally, we're ready to implement `GitRepository::find` function:
```rust
// src/repository.rs

    impl GitRepository {
    // ...

        /// Try to load a git repo in `working_dir`, if it fails, recursively try parent directory.
        pub fn find(working_dir: &Path) -> Result<Self, String> {
            match DirectoryManager::is_toplevel_directory(working_dir) {
                true => GitRepository::load(working_dir),
                false => {
                    let parent_path = working_dir.parent().ok_or("Not a git repository")?;
                    GitRepository::find(parent_path)
                }
            }
        }

    //...
    }
```
If given directory is a top-top level directory, load it. Otherwise, Go to parent directory and check it.
As usual I added integration tests to verify `GitRepository` works fine or not. Take a look at them [here]()ADD_LINK_HERE.

## what to do next
1. Consider using a better error type instead of `String`.
2. I don't like the name of `DirectoryManager`. Choose a better name for it.