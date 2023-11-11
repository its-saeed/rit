# Add ls-tree

While we’re at it, let’s add the ls-tree command. It’s so easy there’s no reason not to. git ls-tree \[-r] TREE simply prints the contents of a tree, recursively with the -r flag. In recursive:

### Update cli

```rust
    LsTree {
        recursive: bool,
        tree: Sha1,
    },
```

```rust
        .subcommand(
            ClapCommand::new("ls-tree")
                .about("Pretty-print a tree object.")
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .help("Recurse into sub-trees")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("tree")
                        .value_name("TREE")
                        .help("A tree-ish object"),
                ),
        )

```

```rust
    } else if let Some(subcommand) = matches.subcommand_matches("ls-tree") {
        let tree: String = subcommand.get_one::<String>("tree").unwrap().clone();
        let recursive = subcommand.get_flag("recursive");
        Ok(Command::LsTree { tree, recursive })
```

### Implement ls-tree

{% code title="src/main.rs" lineNumbers="true" %}
```rust

fn cmd_ls_tree(tree: &str, recursive: bool, base_url: PathBuf) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;
    if let git_object::GitObject::Tree(tree) = repo.read_object(tree)? {
        for leaf in tree.iter() {
            if recursive {
                match leaf.get_type() {
                    git_object::mode::Type::Tree => {
                        cmd_ls_tree(&leaf.hash, true, base_url.join(&leaf.path))?;
                    }
                    _ => {
                        println!(
                            "{} {}\t{}",
                            leaf.mode,
                            leaf.hash,
                            base_url.join(&leaf.path).display()
                        );
                    }
                };
            } else {
                println!("{}", leaf);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Provided object is not a tree"));
    };

    Ok(())
}

```
{% endcode %}



### Add get\_type and Display to Leaf

<pre class="language-rust" data-title="src/git_object/tree/leaf.rs" data-line-numbers><code class="lang-rust">impl Leaf {
    pub fn new(mode: &#x26;[u8], path: &#x26;[u8], hash: String) -> Result&#x3C;Self, TreeLeafParseError> {
        let mode = Mode::new(String::from_utf8(mode.to_vec())?)?;
        let path = String::from_utf8(path.to_vec())?;

        Ok(Self { mode, path, hash })
    }

    pub fn parse(buf_reader: &#x26;mut impl std::io::BufRead) -> Result&#x3C;Self, TreeLeafParseError> {
        let mut mode = vec![];
        let mode_size = buf_reader
            .read_until(b' ', &#x26;mut mode)
            .context("Failed to read mode")?;

        let mut path = vec![];
        let path_size = buf_reader
            .read_until(b'\x00', &#x26;mut path)
            .context("Failed to read path")?;

        let mut hash = [0_u8; 20];
        buf_reader
            .read_exact(&#x26;mut hash)
            .context("Failed to read sha1 hash")?;

        Self::new(
            &#x26;mode[..mode_size - 1],
            &#x26;path[..path_size - 1],
            hex::encode(hash),
        )
    }

<strong>    pub fn get_type(&#x26;self) -> Type {
</strong><strong>        self.mode.type_
</strong><strong>    }
</strong>}

<strong>impl Display for Leaf {
</strong><strong>    fn fmt(&#x26;self, f: &#x26;mut std::fmt::Formatter&#x3C;'_>) -> std::fmt::Result {
</strong><strong>        write!(f, "{} {}\t{}", self.mode, self.hash, self.path)
</strong><strong>    }
</strong>}
</code></pre>

### Implement Display for Mode and Type

{% code title="src/git_object/tree/mode.rs" lineNumbers="true" %}
```rust

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.type_, self.file_permissions)
    }
}

```
{% endcode %}

{% code title="src/git_object/tree/mode.rs" lineNumbers="true" %}
```rust
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Type::Tree => "tree",
            Type::RegularFile => "blob",
            Type::SymbolicLink => "blob",
            Type::Submodule => "commit",
        };

        write!(f, "{}", str)
    }
}
```
{% endcode %}

### Implement Deref for Tree

To be able to call .iter() on a Tree

```rust

impl Deref for Tree {
    type Target = Vec<Leaf>;

    fn deref(&self) -> &Self::Target {
        &self.leaves
    }
}

```
