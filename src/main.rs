use anyhow::{Ok, Result};
use rit::{git_object, parse_args, repository::GitRepository, Command};

fn main() -> Result<()> {
    let command = parse_args().unwrap();
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
            filename,
            write,
        } => {
            cmd_hash_object(filename, object_type, write)?;
        }
    };

    Ok(())
}

fn cmd_cat_file(object_type: git_object::GitObjectType, object_hash: String) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    let object = git_object::read(&repo, repo.find_object(object_type, object_hash))?;
    print!("{}", object.serialize());
    Ok(())
}

fn cmd_hash_object(
    filename: String,
    object_type: git_object::GitObjectType,
    write: bool,
) -> Result<()> {
    let hash = if write {
        let current_directory = std::env::current_dir()?;
        let repo = GitRepository::find(&current_directory)?;
        git_object::write(repo, filename, object_type)?
    } else {
        let (hash, _) = git_object::create(filename, object_type)?;
        hash
    };

    println!("{hash}");

    Ok(())
}
