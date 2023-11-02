# Create the project structure

## [Create the project structure](https://github.com/its-saeed/rit/commit/fd6fa5295b3b704da2f73b4b4aa87557a5874d0f)

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

## [Add initial dependencies to the project](https://github.com/its-saeed/rit/commit/48fe2c298d9922c64095b1f7e6559bd5249b1a7a)

Add these crates to the project's dependencies:

```toml
[dependencies]
clap = { version = "4.4.6", features = ["cargo"] }
configparser = "3.0.2"
flate2 = "1.0.28"

```
