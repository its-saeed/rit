# A few more refactors and fixes

## Simplify cli module

So far we have a folder separately for this module because initially, I thought this would be a big one! But now we can make it a simple file instead of a hierarchical module. So I removed `src/cli/mod.rs` and moved everything to `src/cli.rs`

## Fix CompressedGitObject

The next fix is changing the `decompress` function. Instead of reading it as a string, we should read it as raw data, otherwise, it would fail. Why? Because we are dealing with a binary format with could potentially contain non-utf8 characters reading it as a string is not the right way to go!

{% code title="src/git_object/compressed.rs" lineNumbers="true" %}
```rust
impl CompressedGitObject {
    pub fn decompress(buf_reader: impl BufRead) -> Result<SerializedGitObject, ObjectParseError> {
        let mut zlib = ZlibDecoder::new(buf_reader);
        let mut buffer = vec![];
        zlib.read_to_end(&mut buffer)?;
        Ok(SerializedGitObject::new(buffer))
    }
}
```
{% endcode %}

Also, a few minor changes needed to serialized.rs file.&#x20;

<pre class="language-rust"><code class="lang-rust">pub struct SerializedGitObject {
<strong>    raw: Vec&#x3C;u8>,
</strong>    pub hash: String,
}
</code></pre>

We need to change the raw from String to Vec\<u8>, therefore `new` function needs an update too. As well as `try_into`:

```rust
impl TryInto<GitObject> for SerializedGitObject {
    type Error = ObjectParseError;

    fn try_into(self) -> Result<GitObject, Self::Error> {
        let mut buffer = self.raw.as_ref();
        let object_header = Header::load(&mut buffer)?;
        GitObject::deserialize(&mut buffer, object_header)
    }
}

```

