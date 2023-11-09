use std::path::Path;

use anyhow::{Ok, Result};
use colored::Colorize;
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
        } => cmd_cat_file(object_type, object_hash)?,
        Command::HashObject {
            object_type,
            file_path,
            write,
        } => cmd_hash_object(&file_path, object_type, write)?,
        Command::Log { commit, n_logs } => cmd_log(commit, n_logs)?,
    };

    Ok(())
}

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

fn cmd_cat_file(object_type: git_object::Type, object_hash: String) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    let object = repo.read_object(&repo.find_object(object_type, object_hash)?)?;
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
