# Let's create a new git in rust!

## 1 - [Create the project structure](https://github.com/its-saeed/rit/commit/fd6fa5295b3b704da2f73b4b4aa87557a5874d0f)
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

## 2 - [Add initial dependencies to the project](https://github.com/its-saeed/rit/commit/48fe2c298d9922c64095b1f7e6559bd5249b1a7a)
Add these crates to the project's dependencies:

```toml
[dependencies]
clap = { version = "4.4.6", features = ["cargo"] }
configparser = "3.0.2"
flate2 = "1.0.28"
sha1 = "0.10.6"

```
## 3 - [Start parsing first command, init](https://github.com/its-saeed/rit/commit/65630f9587a8ccd0f498aea46b11ef668dc3155a)
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


2. Start parsing arguments in `src/cli/mod.rs` like:

We're not going to use any crates for errors right now, Let's just return a simple `String` error.


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
3. `Use` cli structs and functions publicly so we can use them like `rit::parse_args` instead of `rit::cli::parse_args`
```rust
// src/lib.rs

pub use cli::*;
```
4. Use `parse_args` and print the results in main:
```rust
// src/main.rs
use rit::parse_args;

fn main() {
    let command = parse_args().unwrap();
    println!("{:?}", command);
}
```