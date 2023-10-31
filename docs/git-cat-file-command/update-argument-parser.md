# Update argument parser

Our argument parser currently just supports `init` command. Let's add `cat-file` command.

{% code title="src/cli/mod.rs" lineNumbers="true" %}
```rust
#[derive(Debug)]
pub enum Command {
    Init {
        path: String,
    },
    CatFile {
        object_type: GitObjectType,
        object_hash: String,
    },
}
```
{% endcode %}

`CatFile` supposed to accept two parameters. First is the type, and second is the hash.

{% code title="src/cli/mod.rs" lineNumbers="true" %}
```rust
pub fn parse_args() -> Result<Command, ParseArgumentsError> {
    let matches = command!()
        .subcommand(
            ClapCommand::new("init").arg(Arg::new("path").value_name("PATH").required(true)),
        )
        .subcommand(
            ClapCommand::new("cat-file")
                .arg(Arg::new("type").value_name("TYPE").required(true))
                .arg(Arg::new("object").value_name("OBJECT").required(true)),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_matches("init") {
        let path = subcommand.get_one::<String>("path").unwrap().clone();
        Ok(Command::Init { path })
    } else if let Some(subcommand) = matches.subcommand_matches("cat-file") {
        let object_type: String = subcommand.get_one::<String>("type").unwrap().clone();
        let object_hash = subcommand.get_one::<String>("object").unwrap().clone();
        Ok(Command::CatFile {
            object_type: object_type.parse()?,
            object_hash,
        })
    } else {
        Err(anyhow!("Argument parse failed"))?
    }
}

```
{% endcode %}

In line 6 we register a new sub-command `cat-file` with two arguments, type and object. We also make them required.

In line 16 if the user enters `cat-file` as a command to our cli, we extract the type and hash and return them. The compiler complains about line 20 so we need to update `ParseArgumentsError`:

{% code title="src/error.rs" lineNumbers="true" %}
```rust
#[derive(Debug, Error)]
pub enum ParseArgumentsError {
    #[error(transparent)]
    ParseObjectTypeError(#[from] ObjectParseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
```
{% endcode %}

In line 4 we added a new error originating from `ObjectParseError` we implemented in [add-objectparseerror.md](add-objectparseerror.md "mention").
