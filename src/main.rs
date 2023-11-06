use std::path::Path;

use anyhow::{Ok, Result};
use rit::{
    git_object::{self},
    parse_args,
    repository::GitRepository,
    Command,
};

fn main() -> Result<()> {
    let command = parse_args()?;
    match command {
        Command::Init { path } => {
            GitRepository::create(path)?;
        }
        Command::CatFile {
            object_type,
            object_hash,
        } => {
            cmd_cat_file(object_type, object_hash)?;
        }
        Command::HashObject {
            object_type,
            file_path,
            write,
        } => {
            cmd_hash_object(&file_path, object_type, write)?;
        }
    };

    Ok(())
}

fn cmd_cat_file(object_type: git_object::Type, object_hash: String) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    let object = repo.read_object(repo.find_object(object_type, object_hash))?;
    print!("{}", object.serialize());
    Ok(())
}

fn cmd_hash_object(file_path: &Path, object_type: git_object::Type, write: bool) -> Result<()> {
    let hash = if write {
        let current_directory = std::env::current_dir()?;
        let repo = GitRepository::find(&current_directory)?;
        repo.write_object(file_path, object_type)?
    } else {
        let object = GitRepository::create_object(file_path, object_type)?;
        object.hash
    };

    println!("{hash}");

    Ok(())
}
