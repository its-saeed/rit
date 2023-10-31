# Implement read function

{% code title="src/git_object.rs" lineNumbers="true" fullWidth="false" %}
```rust
pub fn read(repo: &GitRepository, sha: String) -> Result<Box<dyn GitObject>, ObjectParseError> {
    let real_file_path = repo.directory_manager.sha_to_file_path(&sha);

    let file = fs::File::open(real_file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut zlib = ZlibDecoder::new(&mut buf_reader);
    let mut buffer = String::new();
    zlib.read_to_string(&mut buffer)?;
    let mut buffer = buffer.as_bytes();
    let object_header = parse_object_file_header(&mut buffer)?;

    match object_header.object_type {
        GitObjectType::Commit => Ok(Box::new(read_commit_object(&mut buffer, object_header)?)),
        GitObjectType::Tree => Ok(Box::new(read_tree_object(&mut buffer, object_header)?)),
        GitObjectType::Tag => Ok(Box::new(read_tag_object(&mut buffer, object_header)?)),
        GitObjectType::Blob => Ok(Box::new(read_blob_object(&mut buffer, object_header)?)),
    }
}
```
{% endcode %}

In line 2 we get the real path of the hash using a function we implemented earlier.&#x20;

Lines 4 to 9 read the content of the object and decompress them. Because they're encoded with zlib.

In line 10 we parse the header, returning an error in the case of a failure. If everything goes well we parse each file based on the type we extracted from the header.

### Parse the header

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
fn parse_object_file_header(
    buf_reader: &mut impl std::io::BufRead,
) -> Result<GitObjectHeader, ObjectParseError> {
    let mut buffer = Vec::new();
    let length = buf_reader
        .read_until(b' ', &mut buffer)
        .context("Failed to read object type")?;
    let object_type = String::from_utf8_lossy(&buffer[..length - 1]);
    let object_type: GitObjectType = object_type.parse()?;

    buffer = Vec::new();
    let length = buf_reader.read_until(b'\x00', &mut buffer)?;
    let object_size = String::from_utf8_lossy(&buffer[..length - 1]);
    let object_size = object_size.parse()?;

    Ok(GitObjectHeader {
        object_type,
        object_size,
    })
}
```
{% endcode %}

The code is pretty straightforward, first, we read the type (as a string) and then we try to parse it to a GitObjectType. Why can we call `parse` here? Because we already implemented the `FromStr` trait for this struct.

Afterward, we try to extract and parse the actual object size.&#x20;

If everything goes well, we return the `GitObjectHeader` we extracted.

### Add a unit test for parse\_object\_file\_header

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust

#[cfg(test)]
mod tests {
    use super::GitObjectType;

    use super::parse_object_file_header;

    #[test]
    fn parse_object_file_header_should_read_correct_header() -> Result<(), anyhow::Error> {
        // 00000000  63 6f 6d 6d 69 74 20 31  30 38 36 00 74 72 65 65  |commit 1086.tree|
        let object_header = hex::decode("636f6d6d697420313038360074726565").unwrap();
        let object_header = parse_object_file_header(&mut object_header.as_ref())?;
        assert_eq!(object_header.object_type, GitObjectType::Commit);
        assert_eq!(object_header.object_size, 1086);

        Ok(())
    }
}
```
{% endcode %}

Pay attention that tests can return `Result` as well. Using ? we can make a test fail if one of our functions returns an error.

### Implement read\_blob\_object function

We have all parts of the `read` function except:

```rust
    match object_header.object_type {
        GitObjectType::Commit => Ok(Box::new(read_commit_object(&mut buffer, object_header)?)),
        GitObjectType::Tree => Ok(Box::new(read_tree_object(&mut buffer, object_header)?)),
        GitObjectType::Tag => Ok(Box::new(read_tag_object(&mut buffer, object_header)?)),
        GitObjectType::Blob => Ok(Box::new(read_blob_object(&mut buffer, object_header)?)),
    }
```

The function is simple, because blobs are simple:

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
fn read_blob_object(
    buf_reader: &mut impl std::io::BufRead,
    object_header: GitObjectHeader,
) -> Result<BlobObject, ObjectParseError> {
    let mut blob = String::new();
    let length = buf_reader.read_to_string(&mut blob)?;
    if length != object_header.object_size {
        return Err(ObjectParseError::MismatchedObjectSize);
    }
    Ok(BlobObject { blob })
}
```
{% endcode %}

We just read the rest of the bytes and check if the read length is the same as the size we got from the header. If it's not, return an error.

We're not going to implement the rest, so just add them as a todo function:

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
fn read_commit_object(
    _buf_reader: &mut impl std::io::BufRead,
    _object_header: GitObjectHeader,
) -> Result<CommitObject, ObjectParseError> {
    todo!()
}

fn read_tree_object(
    _buf_reader: &mut impl std::io::BufRead,
    _object_header: GitObjectHeader,
) -> Result<TreeObject, ObjectParseError> {
    todo!()
}

fn read_tag_object(
    _buf_reader: &mut impl std::io::BufRead,
    _object_header: GitObjectHeader,
) -> Result<TagObject, ObjectParseError> {
    todo!()
}
```
{% endcode %}

We also need to add the rest of the object structs:

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
pub trait GitObject {
    fn get_type() -> GitObjectType
    where
        Self: Sized;

    fn serialize(&self) -> String;
}

pub struct TreeObject;
impl GitObject for TreeObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Tree
    }

    fn serialize(&self) -> String {
        todo!()
    }
}

pub struct CommitObject;
impl GitObject for CommitObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Commit
    }

    fn serialize(&self) -> String {
        todo!()
    }
}

pub struct TagObject;
impl GitObject for TagObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Tag
    }

    fn serialize(&self) -> String {
        todo!()
    }
}
```
{% endcode %}

