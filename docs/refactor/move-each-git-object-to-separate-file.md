# Move each git object to separate file

## Create a module for blob

Because we changed GitObject to enum, we no longer need to say that Blob implements GitObject. It becomes a simple struct.

{% code title="src/git_object/blob.rs" lineNumbers="true" %}
```rust
#[derive(Debug)]
pub struct Blob {
    pub blob: String,
}

impl Blob {
    pub fn serialize(&self) -> String {
        // TODO: Make it memory-friendly
        self.blob.clone()
    }

    pub fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        let mut blob = String::new();
        let length = buf_reader.read_to_string(&mut blob)?;
        if length != object_header.object_size {
            return Err(ObjectParseError::MismatchedObjectSize);
        }
        Ok(Blob { blob })
    }
}

```
{% endcode %}

## Create a module for commit

{% code title="src/git_object/commit.rs" lineNumbers="true" %}
````rust
#[derive(Debug)]
pub struct Commit;

impl Commit {
    pub fn serialize(&self) -> String {
        todo!()
    }

    pub fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        todo!()
    }
}
```
````
{% endcode %}

Do the same for tree and tag objects.
