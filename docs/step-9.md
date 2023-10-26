# [9 - Finish `init` command implementation](https://github.com/its-saeed/rit/commit/cef24c8bcc9fa4bcea5957a69e81670680d0ad2e)

`init` command is supposed to have a path argument. Let's make it mandatory and extract it:
```rust
// src/cli/mod.rs

#[derive(Debug)]
pub enum Command {
    // Change init to have a path field
    Init { path: String },
}

pub fn parse_args() -> Result<Command, String> {
    let matches = command!()
        .subcommand(
            // Make path argument to init command mandatory
            ClapCommand::new("init").arg(Arg::new("path").value_name("PATH").required(true)),
        )
        .get_matches();

    match matches.subcommand_matches("init") {
        Some(subcommand) => {
            // Extract path and pass it to Command::Init
            let path = subcommand.get_one::<String>("path").unwrap().clone();
            Ok(Command::Init { path })
        }
        None => Err("Failed to parse".to_string()),
    }
}

```
Then change the main to respect init command:
```rust
use rit::{parse_args, repository::GitRepository, Command};

fn main() {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => GitRepository::create(path).unwrap(),
    };
}

```
Finally you can run it `cargo run -- init /tmp/test-project`. You'll have a compilation error. Try to fix it or take a look at the [commit](https://github.com/its-saeed/rit/commit/cef24c8bcc9fa4bcea5957a69e81670680d0ad2e)!