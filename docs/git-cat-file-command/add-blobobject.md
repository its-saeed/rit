# Add BlobObject

BlobObject is the first type of the git object we're going to support. The structure is simple:

```rust
// src/git_object.rs

pub struct BlobObject {
    pub blob: String,
}
```

It just has a blob member containing the actual data of the object. Now let's implement `GitObject` for this struct:

```rust
impl GitObject for BlobObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Blob
    }

    fn serialize(&self) -> String {
        // TODO: Make it memory-friendly
        self.blob.clone()
    }
}
```

The serialize function is simple too. Blob objects don't have any special format, so we just return `blob`.
