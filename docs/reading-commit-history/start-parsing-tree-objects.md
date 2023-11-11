# Start parsing tree objects

Read the [original source](https://wyag.thb.lt/#checkout) to find out what a tree is.

## Parsing trees <a href="#org1f37f5d" id="org1f37f5d"></a>

Unlike tags and commits, tree objects are binary objects, but their format is actually quite simple. A tree is the concatenation of records of the format:

```
[mode] space [path] 0x00 [sha-1]
```

* `[mode]` is up to six bytes and is an octal representation of a file mode, stored in ASCII. For example, 100644 is encoded with byte values 49 (ASCII “1”), 48 (ASCII “0”), 48, 54, 52, 52. The first two digits encode the file type (file, directory, symlink, or submodule), and the last four are the permissions.
* It’s followed by 0x20, an ASCII space;
* Followed by the null-terminated (0x00) path;
* Followed by the object’s SHA-1 in binary encoding, on 20 bytes.

Creating a new hierarchical tree module

src/git\_object/tree.rs is not a good place to implement tree parsing. Because it's going to be a big module again. So I created a new folder `src/git_object/tree/`and moved `tree.rs` to mod.rs to this folder.

```bash
mkdir src/git_object/tree
mv src/git_object/tree.rs src/git_object/tree/mod.rs
```

### Update Tree

```rust
#[derive(Debug)]
pub struct Tree {
    pub leaves: Vec<Leaf>,
}
```

### Add leaf module to tree

Create a new file, leaf.rs in the tree folder. Add the struct:

```rust
#[derive(Debug)]
pub struct Leaf {
    pub mode: Mode,
    pub path: String,
    pub hash: String,
}
```

Let's implement the parse function of Leaf:

```rust
    pub fn parse(buf_reader: &mut impl std::io::BufRead) -> Result<Self, TreeLeafParseError> {
        let mut mode = vec![];
        let mode_size = buf_reader
            .read_until(b' ', &mut mode)
            .context("Failed to read mode")?;

        let mut path = vec![];
        let path_size = buf_reader
            .read_until(b'\x00', &mut path)
            .context("Failed to read path")?;

        let mut hash = [0_u8; 20];
        buf_reader
            .read_exact(&mut hash)
            .context("Failed to read sha1 hash")?;

        Self::new(
            &mode[..mode_size - 1],
            &path[..path_size - 1],
            hex::encode(hash),
        )
    }
```

And of course the new function:

```rust
    pub fn new(mode: &[u8], path: &[u8], hash: String) -> Result<Self, TreeLeafParseError> {
        let mode = Mode::new(String::from_utf8(mode.to_vec())?)?;
        let path = String::from_utf8(path.to_vec())?;

        Ok(Self { mode, path, hash })
    }
```

### Add mode module to tree

Create a new file mode.rs in the tree folder, and add the struct:

{% code title="src/git_object/tree/mode.rs" lineNumbers="true" %}
```rust
#[derive(Debug)]
pub struct Mode {
    pub type_: Type,
    file_permissions: String,
}
```
{% endcode %}

Add a `new` function to this struct:

{% code title="src/git_object/tree/mode.rs" lineNumbers="true" %}
```rust
impl Mode {
    pub fn new(mode: String) -> Result<Self, TreeLeafParseError> {
        let mode = if mode.len() == 5 {
            format!("0{}", mode)
        } else {
            mode
        };
        let type_length: usize = if mode.len() == 5 { 1 } else { 2 };
        let type_str = &mode[0..type_length];
        let type_: Type = type_str.parse()?;
        Ok(Self {
            file_permissions: mode,
            type_,
        })
    }
}
```
{% endcode %}

We just convert the raw mode string to type and file permissions. Because the raw string can have 5 or 6 characters, we may need to append a 0 at the beginning.&#x20;

In line 11, we try to convert the type string to a Type object. We will add Type enum in the next section.

### Add Type

{% code title="src/git_object/tree/mode.rs" lineNumbers="true" %}
```rust
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Tree = 4,
    RegularFile = 10,
    SymbolicLink = 12,
    Submodule = 16,
}
```
{% endcode %}

And in order to be able to parse a string to Type we need to implement FromStr for this enum:

{% code title="src/git_object/tree/mode.rs" lineNumbers="true" %}
```rust

impl FromStr for Type {
    type Err = TreeLeafParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "04" | "4" => Ok(Self::Tree),
            "10" => Ok(Self::RegularFile),
            "12" => Ok(Self::SymbolicLink),
            "16" => Ok(Self::Submodule),
            _ => Err(TreeLeafParseError::InvalidFileMode),
        }
    }
}
```
{% endcode %}

### Adding the error type

Don't forget to add a new error type:

{% code title="src/error/git_object.rs" lineNumbers="true" %}
```rust

#[derive(Debug, Error)]
pub enum TreeLeafParseError {
    #[error("Invalid file mode")]
    InvalidFileMode,

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

```
{% endcode %}

### Implement deserialize function for Tree

```rust
impl Tree {
    pub fn serialize(&self) -> String {
        todo!()
    }

    pub fn deserialize(
        mut buf_reader: impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        let mut leaves = vec![];
        loop {
            match Leaf::parse(&mut buf_reader) {
                Ok(leaf) => leaves.push(leaf),
                // TODO: Fix this
                Err(_) => break,
            }
        }

        Ok(Self { leaves })
    }
}
```

### Update GitObject

<pre class="language-rust"><code class="lang-rust">
impl GitObject {
    pub fn serialize(&#x26;self) -> String {
        match self {
            GitObject::Commit(commit) => commit.serialize(),
            GitObject::Blob(blob) => blob.serialize(),
            GitObject::Tag(_) => todo!(),
            GitObject::Tree(_) => todo!(),
        }
    }

    fn deserialize(
        buf_reader: &#x26;mut impl std::io::BufRead,
        object_header: Header,
    ) -> Result&#x3C;Self, ObjectParseError> {
        match object_header.object_type {
            Type::Commit => Ok(Self::Commit(Commit::deserialize(
                buf_reader,
                object_header,
            )?)),
<strong>            Type::Tree => Ok(Self::Tree(Tree::deserialize(buf_reader, object_header)?)),
</strong>            Type::Tag => todo!(),
            Type::Blob => Ok(Self::Blob(Blob::deserialize(buf_reader, object_header)?)),
        }
    }
}

</code></pre>
