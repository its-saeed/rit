# 1 - [Create the project structure](https://github.com/its-saeed/rit/commit/fd6fa5295b3b704da2f73b4b4aa87557a5874d0f)
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