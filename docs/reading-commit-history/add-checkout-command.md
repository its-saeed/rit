# Add checkout command

`git checkout` simply instantiates a commit in the worktree. We’re going to oversimplify the actual git command to make our implementation clear and understandable. We’re also going to add a few safeguards. Here’s how our version of checkout will work:

* It will take two arguments: a commit, and a directory. Git checkout only needs a commit.
* It will then instantiate the tree in the directory, if and only if the directory is empty.&#x20;

### Update cli module

```rust
    Checkout {
        commit: Sha1,
        path: String,
    },
}
```

```rust
        .subcommand(
            ClapCommand::new("checkout")
                .about("Checkout a commit inside a directory")
                .arg(
                    Arg::new("commit")
                        .value_name("COMMIT")
                        .help("The commit or tree to checkout"),
                )
                .arg(
                    Arg::new("path")
                        .value_name("PATH")
                        .help("An EMPTY directory to checkout on"),
                ),
        )
```

```rust
    } else if let Some(subcommand) = matches.subcommand_matches("checkout") {
        let commit: String = subcommand.get_one::<String>("commit").unwrap().clone();
        let path = subcommand.get_one::<String>("path").unwrap().clone();
        Ok(Command::Checkout { commit, path })
    } else {

```

### Update main

```rust
        Command::Checkout { commit, path } => cmd_checkout(commit, PathBuf::from(path))?,

```

### Add cmd\_checkout

```rust

fn cmd_checkout(commit: String, path: PathBuf) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    // Read the object
    let object = {
        let object = repo.read_object(&commit)?;
        if let git_object::GitObject::Commit(commit) = object {
            repo.read_object(
                commit
                    .get_value("tree")
                    .ok_or(anyhow::anyhow!("No tree entry found in the object"))?,
            )?
        } else {
            object
        }
    };

    // Create the path if doesn't exist
    if path.exists() {
        if path.is_file() {
            return Err(anyhow::anyhow!("{} not a directory!", path.display()));
        } else if path.is_dir() && path.read_dir()?.next().is_some() {
            return Err(anyhow::anyhow!("{} not a empty!", path.display()));
        }
    } else {
        std::fs::create_dir_all(&path)?;
    }

    // checkout the tree
    if let git_object::GitObject::Tree(tree) = object {
        tree_checkout(&repo, tree, path)?;
    }

    Ok(())
}

```

### Add tree\_checkout function

{% code title="src/main.rs" lineNumbers="true" %}
```rust
fn tree_checkout(repo: &GitRepository, tree: git_object::Tree, base_path: PathBuf) -> Result<()> {
    for leaf in tree.iter() {
        match leaf.get_type() {
            git_object::mode::Type::Tree => {
            // If it's a tree, create the directory, and read the tree recursively.
                std::fs::create_dir(base_path.join(&leaf.path))?;
                let object = repo.read_object(&leaf.hash)?;
                if let git_object::GitObject::Tree(tree) = object {
                    tree_checkout(&repo, tree, base_path.join(&leaf.path))?;
                } else {
                    return Err(anyhow::anyhow!("Invalid tree object"));
                }
            }
            // Otherwise, write the file
            git_object::mode::Type::RegularFile => {
                let object = repo.read_object(&leaf.hash)?;
                if let git_object::GitObject::Blob(blob) = object {
                    std::fs::write(base_path.join(&leaf.path), blob.serialize())?;
                } else {
                    return Err(anyhow::anyhow!("Invalid blob object"));
                };
            }
            git_object::mode::Type::SymbolicLink => todo!(),
            git_object::mode::Type::Submodule => todo!(),
        };
    }

    Ok(())
}

```
{% endcode %}
