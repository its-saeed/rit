use anyhow::{Ok, Result};
use rit::{parse_args, repository::GitRepository, Command};

fn main() -> Result<()> {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => GitRepository::create(path)?,
    };

    Ok(())
}
