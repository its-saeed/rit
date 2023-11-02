# The hash-object command

We will want to put our _own_ data in our repositories, though. `hash-object` is basically the opposite of `cat-file`: it reads a file and computes its hash as an object, either storing it in the repository (if the -w flag is passed) or just printing its hash.

**After this step, we need to refactor the code again like before. Refactoring makes it easy to develop and add more features to the code.**&#x20;

### Add hash-object to argument parser

Let's add a new Command entry:

<pre class="language-rust" data-title="src/cli/mod.rs" data-line-numbers><code class="lang-rust">#[derive(Debug)]
pub enum Command {
    Init {
        path: String,
    },
    CatFile {
        object_type: GitObjectType,
        object_hash: String,
    },
<strong>    HashObject {
</strong><strong>        object_type: GitObjectType,
</strong><strong>        filename: String,
</strong><strong>        write: bool,
</strong><strong>    },
</strong>}

</code></pre>

If `write` is set (i.e. -w or --write is passed to the application) we write the object, otherwise, we print out the hash.

Now let's update `parse_args()`

<pre class="language-rust" data-title="" data-line-numbers><code class="lang-rust">pub fn parse_args() -> Result&#x3C;Command, ParseArgumentsError> {
    let matches = command!()
        .subcommand(
            ClapCommand::new("init").arg(Arg::new("path").value_name("PATH").required(true)),
        )
        .subcommand(
            ClapCommand::new("cat-file")
                .arg(Arg::new("type").value_name("TYPE").required(true))
                .arg(Arg::new("object").value_name("OBJECT").required(true)),
        )
<strong>        .subcommand(
</strong><strong>            ClapCommand::new("hash-object")
</strong><strong>                .about("Compute object ID and optionally creates a blob from a file")
</strong><strong>                .arg(
</strong><strong>                    Arg::new("write")
</strong><strong>                        .short('w')
</strong><strong>                        .long("write")
</strong><strong>                        .action(ArgAction::SetTrue),
</strong><strong>                )
</strong><strong>                .arg(
</strong><strong>                    Arg::new("type")
</strong><strong>                        .value_name("TYPE")
</strong><strong>                        .short('t')
</strong><strong>                        .long("type")
</strong><strong>                        .default_value("blob"),
</strong><strong>                )
</strong><strong>                .arg(Arg::new("file").value_name("FILE").required(true)),
</strong>        )
        .get_matches();
</code></pre>

In order to make `write` a flag, we need to add line 18. We also set a default value for `type` . Finally, we need to add another else if statement to check if we get a `hash-object` subcommand:

```rust
  } else if let Some(subcommand) = matches.subcommand_matches("hash-object") {
        let filename: String = subcommand.get_one::<String>("file").unwrap().clone();
        let object_type = subcommand.get_one::<String>("type").unwrap();
        let write = subcommand.get_flag("write");
        Ok(Command::HashObject {
            filename,
            object_type: object_type.parse()?,
            write,
        })
    } else {
```

Now let's update main.rs

<pre class="language-rust" data-title="src/main.rs" data-line-numbers><code class="lang-rust">fn main() -> Result&#x3C;()> {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => {
            GitRepository::create(path)?;
        }
        Command::CatFile {
            object_type,
            object_hash,
        } => {
            cmd_cat_file(object_type, object_hash)?;
        }
<strong>        Command::HashObject {
</strong><strong>            object_type,
</strong><strong>            filename,
</strong><strong>            write,
</strong><strong>        } => {
</strong><strong>            cmd_hash_object(filename, object_type, write)?;
</strong><strong>        }
</strong>    };

    Ok(())
}

</code></pre>

If the command is HashObject we call `cmd_hash_object` function:

{% code title="" lineNumbers="true" %}
```rust
fn cmd_hash_object(
    filename: String,
    object_type: git_object::GitObjectType,
    write: bool,
) -> Result<()> {
    let hash = if write {
        let current_directory = std::env::current_dir()?;
        let repo = GitRepository::find(&current_directory)?;
        git_object::write(repo, filename, object_type)?
    } else {
        let (hash, _) = git_object::create(filename, object_type)?;
        hash
    };

    println!("{hash}");

    Ok(())
}
```
{% endcode %}

Here if the `write` flag is set, we create a repository and write the object. Otherwise, we just create an object and get the hash. In any case, we print the hash.

### write Function

Add a new function to the git\_object module. It will call the create function we're going to add later:

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
pub fn write(
    repo: GitRepository,
    filename: String,
    object_type: GitObjectType,
) -> Result<String, ObjectCreateError> {
    let (hash, data) = create(filename, object_type)?;
    let file_path = repo.directory_manager.sha_to_file_path(&hash);

    std::fs::create_dir_all(
        file_path
            .parent()
            .context("Failed to get the parent directory")?,
    )?;

    let mut z = ZlibEncoder::new(data.as_bytes(), Compression::fast());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer)?;
    std::fs::write(file_path, buffer)?;
    Ok(hash)
}
```
{% endcode %}

First, we call `create` to create a new hash object. It is supposed to return the hash and the encoded data.&#x20;

In line 7 we convert the hash to the actual file path using `sha_to_file_path` we implemented earlier.&#x20;

Then in line 9, we create the directory containing the object file.

Lines 15-17 encode the object using zlib and finally, we write it in line 18 and we return the hash.

### create function

Now let's add the create function. First, we read the file and based on the type, we call `GitObject::serialize` to get the serialized data.&#x20;

In line 18 we write the header and serialized data to a vector through a BufWriter object. As you can see, in line 21, we convert object\_type to string. It implies that we need to implement `ToString` trait for this enum. We do it later.

In line 30 we compute the hash using sha1\_smol crate and return the hash and object.

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
pub fn create(
    filename: String,
    object_type: GitObjectType,
) -> Result<(String, String), ObjectCreateError> {
    let input_data = std::fs::read_to_string(filename)?;
    let serialized = match object_type {
        GitObjectType::Commit => todo!(),
        GitObjectType::Tree => todo!(),
        GitObjectType::Tag => todo!(),
        GitObjectType::Blob => {
            let object = BlobObject { blob: input_data };
            object.serialize()
        }
    };

    let buffer = Vec::<u8>::new();
    let mut buf_writer = BufWriter::new(buffer);
    write!(
        buf_writer,
        "{} {}\x00{}",
        object_type.to_string(),
        serialized.len(),
        serialized
    )?;

    buf_writer.flush()?;
    let buffer = buf_writer
        .into_inner()
        .context("Failed to take buffer out of buf writer")?;
    let hash = sha1_smol::Sha1::from(&buffer).hexdigest();
    Ok((hash, String::from_utf8(buffer)?))
}
```
{% endcode %}

So please don't forget to add `sha1_smol` crate to your Cargo.toml file under \[dependencies] section:

```toml
sha1_smol = { version = "1.0.0", features = ["std"] }
```

### Implement ToString for GitObjectType

{% code title="src/git_object.rs" lineNumbers="true" %}
```rust
impl ToString for GitObjectType {
    fn to_string(&self) -> String {
        match self {
            GitObjectType::Commit => "commit".to_string(),
            GitObjectType::Tree => "tree".to_string(),
            GitObjectType::Tag => "tag".to_string(),
            GitObjectType::Blob => "blob".to_string(),
        }
    }
}
```
{% endcode %}

### Add ObjectCreateError

{% code title="src/error.rs" lineNumbers="true" %}
```rust
#[derive(Debug, Error)]
pub enum ObjectCreateError {
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
```
{% endcode %}
