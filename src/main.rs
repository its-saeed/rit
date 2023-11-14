use std::path::PathBuf;

use anyhow::Result;
use rit::{
    executer::{
        cmd_cat_file, cmd_checkout, cmd_hash_object, cmd_init, cmd_log, cmd_ls_tree, cmd_show_ref,
        cmd_tag,
    },
    parse_args, Command,
};

fn main() -> Result<()> {
    let command = parse_args()?;
    match command {
        Command::Init { path } => cmd_init(path),
        Command::CatFile {
            object_type,
            object_hash,
        } => cmd_cat_file(object_type, object_hash),
        Command::HashObject {
            object_type,
            file_path,
            write,
        } => cmd_hash_object(&file_path, object_type, write),
        Command::Log { commit, n_logs } => cmd_log(commit, n_logs),
        Command::LsTree { recursive, tree } => cmd_ls_tree(&tree, recursive, PathBuf::new()),
        Command::Checkout { commit, path } => cmd_checkout(commit, PathBuf::from(path)),
        Command::ShowRef => cmd_show_ref(),
        Command::Tag { command } => cmd_tag(command),
    }
}
