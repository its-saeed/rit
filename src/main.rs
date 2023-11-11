use std::path::{Path, PathBuf};

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
        Command::LsTree { recursive, tree } => cmd_ls_tree(&tree, recursive, PathBuf::new())?,
        Command::Checkout { commit, path } => cmd_checkout(commit, PathBuf::from(path))?,
    };

    Ok(())
}

fn cmd_checkout(commit: String, path: PathBuf) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    // Read the object
    let object = {
        let object = repo.read_object(&commit)?;
        if let git_object::GitObject::Commit(commit) = object {
            repo.read_object(
                commit
                    .get_value("tree")
                    .ok_or(anyhow::anyhow!("No tree entry found in the object"))?,
            )?
        } else {
            object
        }
    };

    // Create the path if doesn't exist
    if path.exists() {
        if path.is_file() {
            return Err(anyhow::anyhow!("{} not a directory!", path.display()));
        } else if path.is_dir() && path.read_dir()?.next().is_some() {
            return Err(anyhow::anyhow!("{} not a empty!", path.display()));
        }
    } else {
        std::fs::create_dir_all(&path)?;
    }

    // checkout the tree
    if let git_object::GitObject::Tree(tree) = object {
        tree_checkout(&repo, tree, path)?;
    }

    Ok(())
}

fn tree_checkout(repo: &GitRepository, tree: git_object::Tree, base_path: PathBuf) -> Result<()> {
    for leaf in tree.iter() {
        match leaf.get_type() {
            git_object::mode::Type::Tree => {
                std::fs::create_dir(base_path.join(&leaf.path))?;
                let object = repo.read_object(&leaf.hash)?;
                if let git_object::GitObject::Tree(tree) = object {
                    tree_checkout(&repo, tree, base_path.join(&leaf.path))?;
                } else {
                    return Err(anyhow::anyhow!("Invalid tree object"));
                }
            }
            git_object::mode::Type::RegularFile => {
                let object = repo.read_object(&leaf.hash)?;
                if let git_object::GitObject::Blob(blob) = object {
                    std::fs::write(base_path.join(&leaf.path), blob.serialize())?;
                } else {
                    return Err(anyhow::anyhow!("Invalid blob object"));
                };
            }
            git_object::mode::Type::SymbolicLink => todo!(),
            git_object::mode::Type::Submodule => todo!(),
        };
    }

    Ok(())
}

fn cmd_ls_tree(tree: &str, recursive: bool, base_url: PathBuf) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;
    if let git_object::GitObject::Tree(tree) = repo.read_object(tree)? {
        for leaf in tree.iter() {
            if recursive {
                match leaf.get_type() {
                    git_object::mode::Type::Tree => {
                        cmd_ls_tree(&leaf.hash, true, base_url.join(&leaf.path))?;
                    }
                    _ => {
                        println!(
                            "{} {}\t{}",
                            leaf.mode,
                            leaf.hash,
                            base_url.join(&leaf.path).display()
                        );
                    }
                };
            } else {
                println!("{}", leaf);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Provided object is not a tree"));
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
            println!("Tree: {}", c.get_value("tree").unwrap());
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
