# Implement show-ref command

## What a ref is, and the show-ref command

[original-source](https://wyag.thb.lt/#cmd-show-ref)

## Update cli module

<pre class="language-rust" data-title="src/cli.rs" data-line-numbers><code class="lang-rust">#[derive(Debug)]
pub enum Command {
    // ...
<strong>    ShowRef,
</strong>}
</code></pre>

{% code title="src/cli.rs" lineNumbers="true" %}
```rust
        .subcommand(ClapCommand::new("show-ref").about("List references."))

```
{% endcode %}

{% code title="src/cli.rs" lineNumbers="true" %}
```rust
} else if let Some(_) = matches.subcommand_matches("show-ref") {
        Ok(Command::ShowRef)
    } else {
```
{% endcode %}

## Update directory\_manager

Let's add a PathBuf to `DirectoryManager` for the refs folder:

<pre class="language-rust" data-title="src/directory_manager.rs" data-line-numbers><code class="lang-rust">#[derive(Debug, Clone)]
pub struct DirectoryManager {
    pub work_tree: PathBuf,
    pub dot_git_path: PathBuf,
    pub config_file: PathBuf,
    pub description_file: PathBuf,
    pub head_file: PathBuf,
    pub branches_path: PathBuf,
    pub objects_path: PathBuf,
<strong>    pub refs_path: PathBuf,
</strong>    pub refs_tags_path: PathBuf,
    pub refs_heads_path: PathBuf,
}
</code></pre>

<pre class="language-rust" data-title="src/directory_manager.rs" data-line-numbers><code class="lang-rust">    pub fn new&#x3C;T: Into&#x3C;PathBuf>>(base_path: T) -> Self {
        let base_path: PathBuf = base_path.into();
        let dot_git_path = base_path.join(".git");

        Self {
            work_tree: base_path,
            config_file: dot_git_path.join("config"),
            description_file: dot_git_path.join("description"),
            head_file: dot_git_path.join("HEAD"),
            branches_path: dot_git_path.join("branches"),
            objects_path: dot_git_path.join("objects"),
<strong>            refs_path: dot_git_path.join("refs"),
</strong>            refs_tags_path: dot_git_path.join("refs").join("tags"),
            refs_heads_path: dot_git_path.join("refs").join("heads"),
            dot_git_path,
        }
    }
</code></pre>

## Move repository module to a separate folder

```bash
mkdir src/repository
cp src/repository.rs src/repository/
```

## Add new module to repository module

Add a new file, `src/repository/refs.rs`

This file is going to have our functions and structs to help us deal with refs:

{% code title="src/repository/refs.rs" lineNumbers="true" %}
```rust
#[derive(Debug)]
pub struct Ref {
    pub hash: String,
    pub path: PathBuf,
}
```
{% endcode %}

### Add resolve\_ref function to refs module

getsThis function gets the path to .git folder and and a path to a ref and tries to resolve it to a hash:

{% code title="src/repository/refs.rs" lineNumbers="true" %}
```rust
pub fn resolve_ref(dot_git_path: &Path, ref_path: &Path) -> Result<String, ResolveRefError> {
    if ref_path.is_file() == false {
        return Err(ResolveRefError::RelativePathIsNotAFile(format!(
            "{}",
            ref_path.display()
        )));
    }

    let ref_value = fs::read_to_string(ref_path)?;
    let ref_value = ref_value.trim_end();

    if ref_value.starts_with("ref: ") {
        return resolve_ref(
            dot_git_path,
                                                // Skip ref:
            &dot_git_path.join(&PathBuf::from(&ref_value[5..])),
        );
    }

    Ok(ref_value.to_string())
```
{% endcode %}

If the given file contains a hash, it returns it, otherwise, it recursively tries to resolve it again.

### Add list\_refs function

Given a path(directory), this function list all of the refs in the folder and all of its sub-directories.

{% code title="src/repository/refs.rs" lineNumbers="true" %}
```rust
pub fn list_refs(dot_git_path: &Path, path: &Path) -> Result<Vec<Ref>, ResolveRefError> {
    let mut refs = vec![];

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let out = list_refs(dot_git_path, &entry_path)?;
            refs.extend(out);
        } else {
            let hash = resolve_ref(dot_git_path, &entry_path)?;
            refs.push(Ref {
                hash,
                path: entry_path,
            });
        }
    }

    Ok(refs)
}
```
{% endcode %}

{% code title="src/error/repository.rs" lineNumbers="true" %}
```rust
#[derive(Debug, Error)]
pub enum ResolveRefError {
    #[error("Relative path {0} is not a file")]
    RelativePathIsNotAFile(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
```
{% endcode %}

### Add resolve\_ref and list\_refs to GitRepository

The resolve\_ref function is simple. We just call refs::resolve\_ref function with appropriate arguments.

{% code title="src/repository/mod.rs" lineNumbers="true" %}
```rust
    pub fn resolve_ref(&self, ref_relative_path: &str) -> Result<String, ResolveRefError> {
        let ref_path = self.directory_manager.dot_git_path.join(ref_relative_path);
        refs::resolve_ref(&self.directory_manager.dot_git_path, &ref_path)
    }
```
{% endcode %}

In list\_refs function, we do one more step. Because the git itself prints the ref paths in the relative manner, we convert the results to relative ones.&#x20;

{% code title="src/repository/mod.rs" lineNumbers="true" %}
```rust
    pub fn list_refs(&self) -> Result<Vec<refs::Ref>, ResolveRefError> {
        let refs = refs::list_refs(
            &self.directory_manager.dot_git_path,
            &self.directory_manager.refs_path,
        )?;
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
```
{% endcode %}

I also added this function to GitRepository for convenience.&#x20;

```rust
    pub fn new(config: GitConfig, directory_manager: DirectoryManager) -> Self {
        Self {
            config,
            directory_manager,
        }
    }
```

### Update main.rs

<pre class="language-rust" data-title="src/main.rs" data-line-numbers><code class="lang-rust">fn main() -> Result&#x3C;()> {
    let command = parse_args()?;
    match command {
        Command::Init { path } => {
            GitRepository::create(path)?;
        }
        Command::CatFile {
            object_type,
            object_hash,
        } => cmd_cat_file(object_type, object_hash)?,
        Command::HashObject {
            object_type,
            file_path,
            write,
        } => cmd_hash_object(&#x26;file_path, object_type, write)?,
        Command::Log { commit, n_logs } => cmd_log(commit, n_logs)?,
        Command::LsTree { recursive, tree } => cmd_ls_tree(&#x26;tree, recursive, PathBuf::new())?,
        Command::Checkout { commit, path } => cmd_checkout(commit, PathBuf::from(path))?,
<strong>        Command::ShowRef => cmd_show_ref()?,
</strong>    };

    Ok(())
}
</code></pre>

{% code title="src/main.rs" lineNumbers="true" %}
```rust

fn cmd_show_ref() -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;
    let refs = repo.list_refs()?;
    for ref_item in refs {
        println!("{}", ref_item);
    }

    Ok(())
}
```
{% endcode %}

Implement Display for Ref to be able to write line 7 in the previous code:

{% code title="src/repository/refs.rs" lineNumbers="true" %}
```rust
impl Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.hash, self.path.display())
    }
}
```
{% endcode %}
