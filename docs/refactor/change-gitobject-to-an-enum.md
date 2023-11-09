# Change GitObject to an enum

We implemented `GitObject` as a trait. For the sake of simplicity I'm going to change to an enum:

```rust
#[derive(Debug)]
pub enum GitObject {
    Commit(Commit),
    Blob(Blob),
    Tag(Tag),
    Tree(Tree),
}
```

As you may know, enum type is a complex data type in Rust and each entry can hold a datatype.&#x20;

Let's implement serialize and deserialize for this enum:

```rust
impl GitObject {
    pub fn serialize(&self) -> String {
        match self {
            GitObject::Commit(commit) => commit.serialize(),
            GitObject::Blob(blob) => blob.serialize(),
            GitObject::Tag(_) => todo!(),
            GitObject::Tree(_) => todo!(),
        }
    }

    fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: Header,
    ) -> Result<Self, ObjectParseError> {
        match object_header.object_type {
            Type::Commit => Ok(Self::Commit(Commit::deserialize(
                buf_reader,
                object_header,
            )?)),
            Type::Tree => todo!(),
            Type::Tag => todo!(),
            Type::Blob => Ok(Self::Blob(Blob::deserialize(buf_reader, object_header)?)),
        }
    }
}

```

The implementations are quite simple. We just call serialize/deserialize of inner data type like Commit or Blob.

### Change TryInto\<GitObject> for SerializedGitObject

{% code title="src/git_object/serialized.rs" lineNumbers="true" %}
```rust
impl TryInto<GitObject> for SerializedGitObject {
    type Error = ObjectParseError;

    fn try_into(self) -> Result<GitObject, Self::Error> {
        let mut buffer = self.raw.as_bytes();
        let object_header = Header::load(&mut buffer)?;
        GitObject::deserialize(&mut buffer, object_header)
    }
}
```
{% endcode %}
