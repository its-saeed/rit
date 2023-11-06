# Add CompressedGitObject

As mentioned earlier, this struct is supposed to deal with compression/decompression.

{% code title="src/git_object/compressed.rs" lineNumbers="true" %}
```rust
pub struct CompressedGitObject {
    pub compressed: Vec<u8>,
}
```
{% endcode %}

In the impl block we implement `decompress` function:

{% code title="" lineNumbers="true" %}
```rust
impl CompressedGitObject {
    pub fn decompress(buf_reader: impl BufRead) -> Result<SerializedGitObject, ObjectParseError> {
        let mut zlib = ZlibDecoder::new(buf_reader);
        let mut buffer = String::new();
        zlib.read_to_string(&mut buffer)?;
        Ok(SerializedGitObject::new(buffer))
    }
}
```
{% endcode %}

It decompresses all of the data in a buf\_reader and returns a `SerializedGitObject`
