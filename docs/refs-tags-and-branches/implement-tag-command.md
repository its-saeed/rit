# Implement tag command

## Update cli module

{% code title="src/cli.rs" lineNumbers="true" %}
```rust

#[derive(Debug)]
pub enum TagSubCommand {
    ListTags,
    CreateTagObject { name: String, object: String },
    CreateLightweightTag { name: String, object: String },
}
```
{% endcode %}

<pre class="language-rust" data-title="src/cli.rs" data-line-numbers><code class="lang-rust">#[derive(Debug)]
pub enum Command {
    ...
<strong>    Tag {
</strong><strong>        command: TagSubCommand,
</strong><strong>    },
</strong>}
</code></pre>

```rust
        .subcommand(
            ClapCommand::new("tag")
                .about("List and create tags")
                .arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("The new tag's name"),
                )
                .arg(
                    Arg::new("object")
                        .value_name("OBJECT")
                        .help("The object the new tag will point to"),
                )
                .arg(
                    Arg::new("tag_object")
                        .short('a')
                        .long("add-tag-object")
                        .requires("name")
                        .help("Whether to create a tag object")
                        .action(ArgAction::SetTrue),
                ),
        )

```

```rust
  } else if let Some(subcommand) = matches.subcommand_matches("tag") {
        let name = subcommand.get_one::<String>("name");
        let object = subcommand.get_one::<String>("object");
        let add_tag_object = subcommand.get_flag("tag_object");
        let add_lightweight_tag = add_tag_object == false && name.is_some();

        if add_tag_object {
            Ok(Command::Tag {
                command: TagSubCommand::CreateTagObject {
                    name: name.unwrap().clone(), // Safe to call unwrap, we specified that if -a presents, name must too.
                    object: object.map(|val| val.clone()).unwrap_or("HEAD".to_string()),
                },
            })
        } else if add_lightweight_tag {
            Ok(Command::Tag {
                command: TagSubCommand::CreateLightweightTag {
                    name: name.unwrap().clone(), // Safe to call unwrap, add_lightweight_tag has a check for presence of name
                    object: object.map(|val| val.clone()).unwrap_or("HEAD".to_string()),
                },
            })
        } else {
            Ok(Command::Tag {
                command: TagSubCommand::ListTags,
            })
        }
    } else {
```

## Update Main

### Let's move all cmd\_ functions to a new module

<pre class="language-rust" data-title="src/lib.rs" data-line-numbers><code class="lang-rust">pub mod cli;
pub mod directory_manager;
pub mod error;
<strong>pub mod executer;
</strong>pub mod git_config;
pub mod git_object;
pub mod repository;

pub use cli::*;
pub use directory_manager::DirectoryManager;
pub use git_object::GitObject;

</code></pre>

the Move all cmd\_ functions from src/main.rs to src/executer.rs. After that, the main will be like this:

{% code title="src/main.rs" lineNumbers="true" %}
```rust
fn main() -> Result<()> {
    let command = parse_args()?;
    match command {
        Command::Init { path } => cmd_init(path),
        Command::CatFile {
            object_type,
            object_hash,
        } => cmd_cat_file(object_type, object_hash),
        Command::HashObject {
            object_type,
            file_path,
            write,
        } => cmd_hash_object(&file_path, object_type, write),
        Command::Log { commit, n_logs } => cmd_log(commit, n_logs),
        Command::LsTree { recursive, tree } => cmd_ls_tree(&tree, recursive, PathBuf::new()),
        Command::Checkout { commit, path } => cmd_checkout(commit, PathBuf::from(path)),
        Command::ShowRef => cmd_show_ref(),
        Command::Tag { command } => cmd_tag(command),
    }
```
{% endcode %}

## Add cmd\_tag to executer module

```rust
fn find_repo_in_current_directory() -> Result<GitRepository, anyhow::Error> {
    let current_directory = std::env::current_dir()?;
    GitRepository::find(&current_directory).context("Failed to create the repo")
}

pub fn cmd_tag(command: TagSubCommand) -> Result<()> {
    let repo = find_repo_in_current_directory()?;
    match command {
        TagSubCommand::ListTags => {
            let tags = repo.list_refs_in(&PathBuf::from("tags"))?;
            for tag in tags {
                println!("{}", tag);
            }
        }
        TagSubCommand::CreateTagObject { name, object } => repo.create_tag_object(name, object)?,
        TagSubCommand::CreateLightweightTag { name, object } => {
            repo.create_lightweight_tag(name, object)?
        }
    };

    Ok(())
}
```

## Update repository module

### Add list\_refs\_in

This function returns all refs in a sub-folder of `refs` folder, specified using the passed argument

```rust
   pub fn list_refs(&self) -> Result<Vec<refs::Ref>, ResolveRefError> {
        self.list_refs_in_absolute(&self.directory_manager.refs_path)
    }

    pub fn list_refs_in(&self, path: &Path) -> Result<Vec<refs::Ref>, ResolveRefError> {
        self.list_refs_in_absolute(&self.directory_manager.refs_path.join(path))
    }

    pub fn list_refs_in_absolute(&self, path: &Path) -> Result<Vec<refs::Ref>, ResolveRefError> {
        let refs = refs::list_refs(&self.directory_manager.dot_git_path, path)?;
        refs.into_iter()
            .map(|ref_item| {
                Ok(refs::Ref {
                    hash: ref_item.hash,
                    path: ref_item
                        .path
                        .strip_prefix(&self.directory_manager.dot_git_path)
                        .context("Failed to strip_prefix")?
                        .to_path_buf(),
                })
            })
            .collect()
    }
}

```

### Create tag-related functions

```rust
// Tag methods
impl GitRepository {
    pub fn create_lightweight_tag(
        &self,
        name: String,
        object: String,
    ) -> Result<(), anyhow::Error> {
        let object = self.find_object(git_object::Type::Tag, object)?;
        std::fs::write(self.directory_manager.refs_tags_path.join(name), object)?;

        Ok(())
    }

    pub fn create_tag_object(&self, name: String, object: String) -> Result<(), anyhow::Error> {
        let object = self.find_object(git_object::Type::Tag, object)?;
        let kvl = BTreeMap::from([
            ("object".to_string(), object),
            ("type".to_string(), "commit".to_string()),
            ("tag".to_string(), name.clone()),
            (
                "tagger".to_string(),
                "Saeed <saeed@zilliqa.com>".to_string(),
            ),
            ("message".to_string(), "This is the message".to_string()),
        ]);

        let tag = Tag {
            kvl: KeyValueList::new(kvl),
        };

        let serialized = SerializedGitObject::try_from(GitObject::Tag(tag))?;

        self.write_object(&serialized)?;
        std::fs::write(
            self.directory_manager.refs_tags_path.join(name),
            serialized.hash,
        )?;
        Ok(())
    }
}
```

### Refactor create\_object and write\_object

Remove both functions and add this:

{% code title="src/repository.rs" lineNumbers="true" %}
```rust
    pub fn create_object(
        file_path: &Path,
        object_type: Type,
    ) -> Result<SerializedGitObject, ObjectCreateError> {
        let mut buf_reader = BufReader::new(File::open(file_path)?);
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer)?;

        let object = match object_type {
            Type::Commit => todo!(),
            Type::Tree => todo!(),
            Type::Tag => todo!(),
            Type::Blob => GitObject::Blob(Blob { blob: buffer }),
        };

        object.try_into()
    }

    pub fn write_object(
        &self,
        serialized_object: &SerializedGitObject,
    ) -> Result<(), anyhow::Error> {
        let file_path = self
            .directory_manager
            .sha_to_file_path(&serialized_object.hash, true)?;

        std::fs::write(file_path, CompressedGitObject::try_from(serialized_object)?)?;

        Ok(())
    }
```
{% endcode %}

To make the compiler happy, remove the serialize function from `SerializedGitObject` and add this function:

{% code title="src/git_object/serialized.rs" lineNumbers="true" %}
```rust
impl TryFrom<GitObject> for SerializedGitObject {
    type Error = ObjectCreateError;

    fn try_from(value: GitObject) -> Result<Self, Self::Error> {
        let (serialized_object, object_type) = match value {
            GitObject::Commit(commit) => (commit.serialize(), git_object::Type::Commit),
            GitObject::Blob(blob) => (blob.serialize(), git_object::Type::Blob),
            GitObject::Tag(tag) => (tag.serialize(), git_object::Type::Tag),
            GitObject::Tree(tree) => (tree.serialize(), git_object::Type::Tree),
        };

        let buffer = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(buffer);

        write!(
            buf_writer,
            "{}{}",
            Header::new(object_type, serialized_object.len()),
            serialized_object
        )?;

        buf_writer.flush()?;
        let buffer = buf_writer
            .into_inner()
            .context("Failed to take buffer out of buf writer")?;

        Ok(SerializedGitObject::new(buffer))
    }
}
```
{% endcode %}

Now cmd\_cat\_file will be:

{% code title="src/executer.rs" lineNumbers="true" %}
```rust
pub fn cmd_hash_object(file_path: &Path, object_type: git_object::Type, write: bool) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;
    let object = GitRepository::create_object(file_path, object_type)?;
    if write {
        repo.write_object(&object)?;
    }

    println!("{}", object.hash);

    Ok(())
}
```
{% endcode %}

## Add tag object

Tag object is completely like commit, so it can be defined like:

{% code title="src/git_object/tag.rs" lineNumbers="true" %}
```rust
// Tag objects are like commits
pub type Tag = super::Commit;
```
{% endcode %}

## Last but not least

<pre class="language-rust" data-title="src/git_object/mod.rs" data-line-numbers><code class="lang-rust">impl GitObject {
    pub fn serialize(&#x26;self) -> String {
        match self {
            GitObject::Commit(commit) => commit.serialize(),
            GitObject::Blob(blob) => blob.serialize(),
<strong>            GitObject::Tag(tag) => tag.serialize(),
</strong><strong>            GitObject::Tree(tree) => tree.serialize(),
</strong>        }
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
            Type::Tree => Ok(Self::Tree(Tree::deserialize(buf_reader, object_header)?)),
<strong>            Type::Tag => Ok(Self::Tag(Tag::deserialize(buf_reader, object_header)?)),
</strong>            Type::Blob => Ok(Self::Blob(Blob::deserialize(buf_reader, object_header)?)),
        }
    }
}

</code></pre>
