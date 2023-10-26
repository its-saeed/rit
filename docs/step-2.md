# 2 - [Add initial dependencies to the project](https://github.com/its-saeed/rit/commit/48fe2c298d9922c64095b1f7e6559bd5249b1a7a)
Add these crates to the project's dependencies:

```toml
[dependencies]
clap = { version = "4.4.6", features = ["cargo"] }
configparser = "3.0.2"
flate2 = "1.0.28"
sha1 = "0.10.6"

```