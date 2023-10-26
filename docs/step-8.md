# 8 - [Let's add some integration tests](https://github.com/its-saeed/rit/commit/cee5c8bdd9356791d9f203024b1edb81a41c8615)
## Good to know
* How to write [integration tests in Rust.](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)

## Implementation
Add a new folder named `tests` beside to `src`.
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
├── target
└── tests
```

Add `tests/test_utils.rs` file to have a place for common test utilities. Although there are [some debates](https://stackoverflow.com/questions/44539729/what-is-an-idiomatic-way-to-have-shared-utility-functions-for-integration-tests) what is the best way to share utilities between tests, I found this way simple and effective. Add your utilities to `tests/test_utilities.rs` and `use test_utilities;` in a test file you need them. Talk is cheap, let's write the code:

We need a function to create a directory manager for us, but we need to create a random name for the `work_tree` directory. That's why we're going to use `uuid` crate. Either add it manually to your Cargo.toml or run `cargo add uuid --features=v4`

Now let's create a `DirectoryManager` object with a randomly generated temp path for its base directory. 
```rust
// tests/test_utilities.rs

#[cfg(test)]
pub mod directory_manager {
    use rit::DirectoryManager;

    use super::general::generate_random_path;
    pub fn create_directory_manager() -> DirectoryManager {
        rit::DirectoryManager::new(generate_random_path())
    }
}

#[cfg(test)]
pub mod general {
    use std::path::PathBuf;

    pub fn generate_random_path() -> PathBuf {
        std::env::temp_dir().join(uuid::Uuid::new_v4().to_string())
    }
}

```
OK! We're ready to add our first integration tests. Why didn't we add them as unit tests? Because we're going to play with filesystem to see if it works in wild!

Add a new file `tests/directory_manager.rs`. Add a test to see what happens if we call `create_directory_tree`. Does it really create some folders for us?

```rust
// To be able to use test_utils. Each file in tests directory will become a separate crate/executable.
mod test_utils;

use test_utils::directory_manager::create_directory_manager;

#[test]
fn directory_tree_should_be_created_successfully() {
    let dir_manager = create_directory_manager();

    dir_manager.create_directory_tree().unwrap();

    assert!(dir_manager.dot_git_path.exists());
    assert!(dir_manager.branches_path.exists());
    assert!(dir_manager.objects_path.exists());
    assert!(dir_manager.refs_heads_path.exists());
    assert!(dir_manager.refs_tags_path.exists());
}
```

Next test! If we have an empty base_directory, `is_dot_git_empty` should happily return true. Because we don't .git
```rust
#[test]
fn if_work_tree_directory_is_empty_is_dot_git_empty_should_return_true() {
    let dir_manager = create_directory_manager();
    std::fs::create_dir_all(&dir_manager.work_tree).unwrap();

    assert!(dir_manager.is_dot_git_empty().unwrap());
}
```

If .git exists but it's empty, `is_dot_git_empty` should return true too.
```rust
#[test]
fn if_dot_git_is_empty_is_dot_git_empty_should_return_true() {
    let dir_manager = DirectoryManager::new(temp_dir().join(Uuid::new_v4().to_string()));

    // To create only .git file. Nothing inside it!
    std::fs::create_dir_all(&dir_manager.dot_git_path).unwrap();

    assert!(dir_manager.is_dot_git_empty().unwrap());
}
```

And finally if .git is not empty, `is_dot_git_empty` should return false:
```rust
#[test]
fn if_dot_git_is_not_empty_is_dot_git_empty_should_return_false() {
    let dir_manager = create_directory_manager();

    // To create .git folder and all of its children.
    dir_manager.create_directory_tree().unwrap();

    assert!(!dir_manager.is_dot_git_empty().unwrap());
}

```

Run them with `cargo test`. Failures! Let's make these tests happy. 

```rust
// src/directory_manager.rs
pub struct DirectoryManager {
    // ...
    // Add some new fields.
    pub branches_path: PathBuf,
    pub objects_path: PathBuf,
    pub refs_tags_path: PathBuf,
    pub refs_heads_path: PathBuf,
}

impl DirectoryManager {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        let base_path: PathBuf = base_path.into();
        let dot_git_path = base_path.join(".git");

        // Initialize them
        Self {
            //...
            branches_path: dot_git_path.join("branches"),
            objects_path: dot_git_path.join("objects"),
            refs_tags_path: dot_git_path.join("refs").join("tags"),
            refs_heads_path: dot_git_path.join("refs").join("heads"),
            dot_git_path,
        }
    }

        pub fn is_dot_git_empty(&self) -> Result<bool, std::io::Error> {
        Ok(!self.dot_git_path.exists() || self.dot_git_path.read_dir()?.next().is_none())
    }

    pub fn create_directory_tree(&self) -> Result<(), std::io::Error> {
        //...
        // Create them!
        fs::create_dir_all(&self.branches_path)?;
        fs::create_dir_all(&self.objects_path)?;
        fs::create_dir_all(&self.refs_heads_path)?;
        fs::create_dir_all(&self.refs_tags_path)?;
        Ok(())
    }
}
```

I also added one more unit test to `directory_manager` module. You can see it [here](https://github.com/its-saeed/rit/commit/cee5c8bdd9356791d9f203024b1edb81a41c8615#diff-1da8b3cfe58b365cf77ad4a6493447271c8bdc9639b70918b88e075d11133a99R81).

Let's add a few tests for `GitRepository` as well. Crate a new file `tests/repository.rs` and a first test:
```rust
#[test]
fn if_project_directory_is_empty_create_should_be_successful() {
    // Create an empty directory
    let project_dir = test_utils::general::generate_random_path();
    std::fs::create_dir_all(&project_dir).unwrap();

    let repo = GitRepository::create(&project_dir).unwrap();

    // Sub-directories should be created.
    let dir_manager = &repo.directory_manager;
    assert!(dir_manager.dot_git_path.exists());
    assert!(dir_manager.branches_path.exists());
    assert!(dir_manager.objects_path.exists());
    assert!(dir_manager.refs_heads_path.exists());
    assert!(dir_manager.refs_tags_path.exists());

    // .git/config should have default configs
    assert_eq!(
        fs::read_to_string(&dir_manager.config_file).unwrap(),
        GitConfig::default_str()
    );
}
```

The rest of the tests are self-explanatory. See them [here](https://github.com/its-saeed/rit/commit/cee5c8bdd9356791d9f203024b1edb81a41c8615#diff-5be74a035aa42d5bef6d92332f6a7c03fd0d8b1db42f4211928a66eba25f99b3)
