# Create a git object module

## Good to know before start

### What are objects?

What is a Git object? At its core, Git is a “content-addressed filesystem”. That means that unlike regular filesystems, where the name of a file is arbitrary and unrelated to that file’s contents, the names of files as stored by Git are mathematically derived from their contents. This has a very important implication: if a single byte of, say, a text file, changes, its internal name will change, too. To put it simply: you don’t modify a file, you create a new file in a different location. Objects are just that: files in the git repository, whose paths are determined by their contents. [source](https://wyag.thb.lt/#objects)

Git uses objects to store quite a lot of things: first and foremost, the actual files it keeps in version control — source code, for example. Commit are objects, too, as well as tags. With a few notable exceptions (which we’ll see later!), almost everything, in Git, is stored as an object. [source](https://wyag.thb.lt/#objects)

The path is computed by calculating the SHA-1 hash of its contents. More precisely, Git renders the hash as a lowercase hexadecimal string, and splits it in two parts: the first two characters, and the rest. It uses the first part as a directory name, the rest as the file name (this is because most filesystems hate having too many files in a single directory and would slow down to a crawl. Git’s method creates 256 possible intermediate directories, hence dividing the average number of files per directory by 256)

### Object format

Before we start implementing the object storage system, we must understand their exact storage format. An object starts with a header that specifies its type: blob, commit, tag or tree (more on that in a second). This header is followed by an ASCII space (0x20), then the size of the object in bytes as an ASCII number, then null (0x00) (the null byte), then the contents of the object. The first 48 bytes of a commit object in Wyag’s repo look like this:

```
00000000  63 6f 6d 6d 69 74 20 31  30 38 36 00 74 72 65 65  |commit 1086.tree|
00000010  20 32 39 66 66 31 36 63  39 63 31 34 65 32 36 35  | 29ff16c9c14e265|
00000020  32 62 32 32 66 38 62 37  38 62 62 30 38 61 35 61  |2b22f8b78bb08a5a|
```

The objects (headers and contents) are stored compressed with zlib.

## Implementation

### Add a simple auxiliary method to DirectoryManager

```rust
// src/directory_manager.rs

    pub fn sha_to_file_path(&self, sha: &str) -> PathBuf {
        self.objects_path.join(&sha[0..2]).join(&sha[2..])
    }
```

This function is supposed to return the absolute path to an object file for a given hash.

Let's add a new unit test for this function:

```rust
// src/directory_manager.rs

   #[test]
    fn sha_to_file_path_should_return_correct_path() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);

        let file_path = dir_manager.sha_to_file_path("e673d1b7eaa0aa01b5bc2442d570a765bdaae751");
        assert_eq!(
            file_path,
            PathBuf::from(format!(
                "{}/.git/objects/e6/73d1b7eaa0aa01b5bc2442d570a765bdaae751",
                PROJECT_DIR
            ))
        );
    }
```

### Create a new module, git\_object

Add a new file to `src` folder named `git_object.rs` and don't forget to add this new module to `lib.rs` as well.

```rust
// src/lib.rs

pub mod git_object;
```

Let's add some new data types to this module. Add a new enum named `GitObjectType`:

```rust
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum GitObjectType {
    Commit,
    Tree,
    Tag,
    Blob,
}

impl FromStr for GitObjectType {
    type Err = ObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "commit" => Ok(GitObjectType::Commit),
            "tree" => Ok(GitObjectType::Tree),
            "tag" => Ok(GitObjectType::Tag),
            "blob" => Ok(GitObjectType::Blob),
            _ => Err(ObjectParseError::InvalidObjectType),
        }
    }
}
```

As you can see we implemented `FromStr` for this type too. Why? Because we'll get the type as a user input. We should check if it's a valid type.&#x20;

We also need a struct named `GitObjectHeader`:

```rust
#[derive(Debug)]
struct GitObjectHeader {
    object_type: GitObjectType,
    object_size: usize,
}
```

### Implement read function

We are now going to add a `read` function to the module to read object files with this signature:

```rust
pub fn read(repo: &GitRepository, sha: String) -> Result<Box<dyn GitObject>, ObjectParseError> {
```

The return type is important. First, we're going to return a [trait object](https://doc.rust-lang.org/beta/reference/types/trait-object.html). Why? Because this function may read and return any of the object types including Commit, Blob, etc.&#x20;

`GitObject` is the general look of an object, defined as a trait.

```rust
pub trait GitObject {
    fn get_type() -> GitObjectType
    where
        Self: Sized;

    fn serialize(&self) -> String;
}
```

All of the objects are supposed to have a type and a `serialize` function.

The simplest object type is Blob. Because they have no actual format. Blobs are user data: the content of every file you put in Git (`main.c`, `logo.png`, `README.md`) is stored as a blob. That makes them easy to manipulate because they have no actual syntax or constraints beyond the basic object storage mechanism: they’re just unspecified data. \[[source](https://wyag.thb.lt/#orgf4d7a3d)]

Let's add them in the next step!
