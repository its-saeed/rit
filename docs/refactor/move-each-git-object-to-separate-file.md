# Move each git object to separate file

## Create a module for blob

{% code title="src/git_object/blob.rs" lineNumbers="true" %}
```rust
use crate::error::ObjectParseError;

use super::{GitObject, Type};

pub struct Blob {
    pub blob: String,
}

impl GitObject for Blob {
    fn get_type() -> Type {
        Type::Blob
    }

    fn serialize(&self) -> String {
        // TODO: Make it memory-friendly
        self.blob.clone()
    }

    fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError>
    where
        Self: Sized,
    {
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
```rust
use super::{GitObject, Type};

pub struct Commit;
impl GitObject for Commit {
    fn get_type() -> Type {
        Type::Commit
    }

    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(
        _buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

```
{% endcode %}

## Create a module for tag&#x20;

{% code title="src/git_object/tag.rs" lineNumbers="true" %}
```rust
use super::{GitObject, Type};

pub struct Tag;
impl GitObject for Tag {
    fn get_type() -> Type {
        Type::Tag
    }

    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(
        _buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

```
{% endcode %}

## Create a module for tree

{% code title="src/git_object/tree.rs" lineNumbers="true" %}
```rust
use super::{GitObject, Type};

pub struct Tag;
impl GitObject for Tag {
    fn get_type() -> Type {
        Type::Tag
    }

    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(
        _buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}

```
{% endcode %}
