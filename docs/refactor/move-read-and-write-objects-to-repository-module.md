# Move read and write objects to repository module

the Currently, `read` and `write` are parts of git\_object module. But I believe they should be part of the repository module. Let's create `read_object` in the repository module.&#x20;

## read\_object function

This function accepts a sha and returns a parsed git object:

{% code title="src/repository.rs" lineNumbers="true" %}
```rust
impl GitRepository {    
    //...

    pub fn read_object(&self, sha: String) -> Result<Box<dyn GitObject>, ObjectParseError> {
        let real_file_path = self.directory_manager.sha_to_file_path(&sha, false)?;
        let file = File::open(real_file_path)?;
        let buf_reader = BufReader::new(file);

        let serialized: SerializedGitObject = CompressedGitObject::decompress(buf_reader)?;

        serialized.try_into()
    }
```
{% endcode %}

We added a new flag to `DirectoryManager::sha_to_file_path` named `create_parent_path`. If we set it to true, it will create the parent directory of the given sha. For now, let's set it to false.

First, we get the absolute path of the sha. Then we open that path and pass a buf\_reader created from that file to `CompressedGitObject::decompress`. It creates a SerializedGitObject for us. In line 11, we call `try_into` function on serialized. What does this function do? It tries to convert the serialized object to the return type of the function, `Box<dyn GitObject>` How is that possible?&#x20;

Actually, we need to implement TryInto for SerializedGitObject:

{% code title="src/git_object/serialized.rs" lineNumbers="true" %}
```rust
impl TryInto<Box<dyn GitObject>> for SerializedGitObject {
    type Error = ObjectParseError;

    fn try_into(self) -> Result<Box<dyn GitObject>, Self::Error> {
        let mut buffer = self.raw.as_bytes();
        let object_header = Header::load(&mut buffer)?;

        match object_header.object_type {
            Type::Commit => Ok(Box::new(Commit::deserialize(&mut buffer, object_header)?)),
            Type::Tree => Ok(Box::new(Tree::deserialize(&mut buffer, object_header)?)),
            Type::Tag => Ok(Box::new(Tag::deserialize(&mut buffer, object_header)?)),
            Type::Blob => Ok(Box::new(Blob::deserialize(&mut buffer, object_header)?)),
        }
    }
}

```
{% endcode %}

## write\_object function

This function accepts a file\_path, creates a git object from that file, and finally writes that git object to the objects directory.&#x20;

{% code title="src/repository.rs" lineNumbers="true" %}
```rust
    pub fn write_object(
        &self,
        file_path: &Path,
        object_type: Type,
    ) -> Result<String, ObjectCreateError> {
        let serialized_object = Self::create_object(file_path, object_type)?;

        let file_path = self
            .directory_manager
            .sha_to_file_path(&serialized_object.hash, true)?;

        std::fs::write(
            file_path,
            CompressedGitObject::try_from(&serialized_object)?,
        )?;
        Ok(serialized_object.hash)
    }
```
{% endcode %}

In line 10, we pass true to create the parent directory of the object.

In line 14, we try to convert a serialized git object to a compressed one, again, we need to implement TryFrom for CompressedGitObject:

{% code title="src/git_object/compressed.rs" lineNumbers="true" %}
```rust
impl TryFrom<&SerializedGitObject> for CompressedGitObject {
    type Error = std::io::Error;

    fn try_from(object: &SerializedGitObject) -> Result<Self, Self::Error> {
        let mut z = ZlibEncoder::new(object.as_ref(), Compression::fast());
        let mut buffer = Vec::new();
        z.read_to_end(&mut buffer)?;
        Ok(Self { compressed: buffer })
    }
}

impl AsRef<[u8]> for CompressedGitObject {
    fn as_ref(&self) -> &[u8] {
        &self.compressed
    }
}

```
{% endcode %}

In line 12 we implement `AsRef` trait for CompressedGitObject so that we are able to pass a reference of it to `std::fs::write` in the previous code block. Just try to remove this trait implementation and see if the compiler lets you build the code.

We also need to implement this trait for SerializedGitObject:

{% code title="src/git_object/serialized.rs" lineNumbers="true" %}
```rust
impl AsRef<[u8]> for SerializedGitObject {
    fn as_ref(&self) -> &[u8] {
        self.raw.as_ref()
    }
}
```
{% endcode %}

## create\_object function

The last function we're going to add to the repository module is `create_object`:

{% code title="src/repository.rs" lineNumbers="true" %}
```rust
    pub fn create_object(
        file_path: &Path,
        object_type: Type,
    ) -> Result<SerializedGitObject, ObjectCreateError> {
        let buf_reader = BufReader::new(File::open(file_path)?);
        SerializedGitObject::serialize(buf_reader, object_type)
    }
```
{% endcode %}

The function is quite simple.

## Change main

To make the compiler happy, we need to change the main.

{% code title="src/main.rs" lineNumbers="true" %}
```rust

fn cmd_cat_file(object_type: git_object::Type, object_hash: String) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    let object = repo.read_object(repo.find_object(object_type, object_hash))?;
    print!("{}", object.serialize());
    Ok(())
}

fn cmd_hash_object(file_path: &Path, object_type: git_object::Type, write: bool) -> Result<()> {
    let hash = if write {
        let current_directory = std::env::current_dir()?;
        let repo = GitRepository::find(&current_directory)?;
        repo.write_object(file_path, object_type)?
    } else {
        let object = GitRepository::create_object(file_path, object_type)?;
        object.hash
    };

    println!("{hash}");

    Ok(())
}

```
{% endcode %}

Instead of using `git_object::write` we use `repo_write_object` in line 15. In line 17 we also call `GitRepository::create_object` instead of `git_object::create`

I changed the cli slightly to have `HashObject` like this:

{% code title="src/cli/mod.rs" lineNumbers="true" %}
```rust
    HashObject {
        object_type: Type,
        file_path: PathBuf,
        write: bool,
    },
```
{% endcode %}

The filename from string type changed to file\_path with PathBuf type.
