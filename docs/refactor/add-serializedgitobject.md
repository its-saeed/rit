# Add SerializedGitObject



We introduce two new structs, `SerializedGitObject` and `CompressedGitObject.` The former is supposed to work with a serialized git object, a header, and an object. However the latter deals with the compression and decompression of git objects to/from zlib format.

## Create SeriazliedGitObject

This struct would be like this. Add new file `src/git_object/serialized.rs`:

{% code title="src/git_object/serialized.rs" lineNumbers="true" %}
```rust
pub struct SerializedGitObject {
    raw: String,
    pub hash: String,
}
```
{% endcode %}

Let's declare two functions for this struct, new and serialize. `new` creates a new `SeriazliedGitObject` and `seralize` gets a buf\_reader and type and creates a serialized git object.

{% code title="src/git_object/serialized.rs" lineNumbers="true" %}
```rust
impl SerializedGitObject {
    pub fn new(raw: String) -> Self {
        Self {
            hash: sha1_smol::Sha1::from(&raw).hexdigest(),
            raw,
        }
    }

    pub fn serialize(
        mut buf_reader: impl BufRead,
        object_type: Type,
    ) -> Result<SerializedGitObject, ObjectCreateError> {
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer)?;
        let serialized = match object_type {
            Type::Commit => todo!(),
            Type::Tree => todo!(),
            Type::Tag => todo!(),
            Type::Blob => {
                let object = Blob { blob: buffer };
                object.serialize()
            }
        };

        let buffer = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(buffer);

        write!(
            buf_writer,
            "{}{}",
            Header::new(object_type, serialized.len()),
            serialized
        )?;

        buf_writer.flush()?;
        let buffer = buf_writer
            .into_inner()
            .context("Failed to take buffer out of buf writer")?;

        Ok(SerializedGitObject::new(String::from_utf8(buffer)?))
    }
}
```
{% endcode %}

Because we implemented `Display` trait for the Header, we can simply use it in line 31. In lines 28-33, we first write the header and then the body of the object to the buffer.
