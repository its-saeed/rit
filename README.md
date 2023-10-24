# Let's create a new git in rust!

## 1 - Create the project structure
Create a new rust project.
```bash
cargo new rit
```

Create a library file named `lib.rs` and add it as a library crate to `Cargo.toml`:

```toml
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "rit"
```

## 2 - Add initial dependencies to the project
Add these crates to the project's dependencies:

```toml
[dependencies]
clap = { version = "4.4.6", features = ["cargo"] }
configparser = "3.0.2"
flate2 = "1.0.28"
sha1 = "0.10.6"

```
## 3 - Create boilerplate code to start parsing arguments
Add parsing arguments basics using `clap` crate. Parse to have a simple CLI realizing the `init` command. No more than just respecting `rit init`

1. Create a new module named `cli`
```rust
// src/lib.rs
pub mod cli;
```

the directory structure should be like:
```
src/
├── cli
│   └── mod.rs
├── lib.rs
└── main.rs
```

2. The initial contents of the src/cli/mod.rs should be:

```rust
pub enum Command {
    Init,
}

pub fn parse_args() -> Result<Subcommand, String> {
    todo!()
}
```
We're not going to use any crates for errors right now, Let's just return a simple `String` error.

## 4 - Parse init command
Change `src/cli/mod.rs` like:

```rust
// 1. Import necessary structs.
use clap::{command, Arg, Command as ClapCommand};

// 2. Add Debug trait to be able to print it out
#[derive(Debug)]
pub enum Command {
    Init,
}

// 3. Implement initial argument parsing.
pub fn parse_args() -> Result<Command, String> {
    let matches = command!()
        .subcommand(ClapCommand::new("init").arg(Arg::new("init")))
        .get_matches();

    match matches.subcommand_matches("init") {
        Some(_subcommand) => Ok(Command::Init),
        None => Err("Failed to parse".to_string()),
    }
}

```
`Use` cli structs and functions publicly so we can use them like `rit::parse_args` instead of `rit::cli::parse_args`
```rust
// src/lib.rs

pub use cli::*;
```
Use `parse_args` and print the results in main:
```rust
// src/main.rs
use rit::parse_args;

fn main() {
    let command = parse_args().unwrap();
    println!("{:?}", command);
}
```