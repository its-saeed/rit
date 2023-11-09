# Update Argument parser

## Add a new subcommand to clap

{% code title="src/cli/mod.rs" lineNumbers="true" %}
```rust
        .subcommand(
            ClapCommand::new("log")
                .about("Display history of given commit")
                .arg(
                    Arg::new("commit")
                        .value_name("COMMIT")
                        .default_value("HEAD")
                        .help("Commit to start at."),
                )
                .arg(
                    Arg::new("n")
                        .value_name("NUMBER")
                        .short('n')
                        .default_value("5")
                        .value_parser(clap::value_parser!(u32))
                        .help("Number of logs to show"),
                ),
        )
```
{% endcode %}

<pre class="language-rust" data-title="src/cli/mod.rs" data-line-numbers><code class="lang-rust">   if let Some(subcommand) = matches.subcommand_matches("init") {
        let path = subcommand.get_one::&#x3C;String>("path").unwrap().clone();
        Ok(Command::Init { path })
    } else if let Some(subcommand) = matches.subcommand_matches("cat-file") {
        let object_type: String = subcommand.get_one::&#x3C;String>("type").unwrap().clone();
        let object_hash = subcommand.get_one::&#x3C;String>("object").unwrap().clone();
        Ok(Command::CatFile {
            object_type: object_type.parse()?,
            object_hash,
        })
    } else if let Some(subcommand) = matches.subcommand_matches("hash-object") {
        let filename: String = subcommand.get_one::&#x3C;String>("file").unwrap().clone();
        let object_type = subcommand.get_one::&#x3C;String>("type").unwrap();
        let write = subcommand.get_flag("write");
        Ok(Command::HashObject {
            file_path: PathBuf::from(filename),
            object_type: object_type.parse()?,
            write,
        })
<strong>    } else if let Some(subcommand) = matches.subcommand_matches("log") {
</strong><strong>        let commit: String = subcommand.get_one::&#x3C;String>("commit").unwrap().clone();
</strong><strong>        let n_logs: u32 = *subcommand.get_one::&#x3C;u32>("n").unwrap();
</strong><strong>        Ok(Command::Log { commit, n_logs })
</strong>    } else {
        Err(anyhow!("Argument parse failed"))?
    }
</code></pre>

<pre class="language-rust" data-title="" data-line-numbers><code class="lang-rust">#[derive(Debug)]
pub enum Command {
    Init {
        path: String,
    },
    CatFile {
        object_type: Type,
        object_hash: String,
    },
    HashObject {
        object_type: Type,
        file_path: PathBuf,
        write: bool,
    },
<strong>    Log {
</strong><strong>        commit: String,
</strong><strong>        n_logs: u32, // Number of logs to show
</strong><strong>    },
</strong>}

</code></pre>

## Update the main to consider the log

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
<strong>        Command::Log { commit, n_logs } => cmd_log(commit, n_logs)?,
</strong>    };

    Ok(())
}
</code></pre>

## Add cmd\_log function

{% code title="" lineNumbers="true" %}
```rust
fn cmd_log(mut commit: String, n_logs: u32) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    for _ in 0..n_logs {
        let commit_hash = repo.find_object(git_object::Type::Commit, commit)?;
        let object = repo.read_object(&commit_hash)?;
        if let git_object::GitObject::Commit(c) = object {
            println!("{} {}", "commit".yellow(), commit_hash.yellow());
            println!("Author: {}", c.get_value("author").unwrap());
            println!();
            println!("  {}", c.get_value("message").unwrap());
            println!();
            commit = match c.get_value("parent") {
                Some(parent) => parent.to_string(),
                None => break,
            };
        } else {
            break;
        }
    }
    Ok(())
}

```
{% endcode %}

In order to display colored text in output, I added a new crate named `colored`

## Update GitRepository::find\_object

Because we set the default value of the commit arg in the log command to HEAD, we need to translate it to the latest commit hash:

```rust
    pub fn find_object(
        &self,
        object_type: git_object::Type,
        name: String,
    ) -> Result<String, anyhow::Error> {
        if name == "HEAD" && object_type == git_object::Type::Commit {
            let hash = fs::read_to_string(self.directory_manager.refs_heads_path.join("master"))
                .context("Can't open refs/heads/master file")?;
            Ok(hash.trim_end().to_string())
        } else {
            Ok(name)
        }
    }
```

As you can see, we can find the latest commit hash in the `refs/heads/master` file
