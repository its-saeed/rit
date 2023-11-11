use std::path::PathBuf;

use anyhow::anyhow;
use clap::{command, Arg, ArgAction, Command as ClapCommand};

use crate::{error::ParseArgumentsError, git_object::Type};

type Sha1 = String;

#[derive(Debug)]
pub enum Command {
    Init {
        path: String,
    },
    CatFile {
        object_type: Type,
        object_hash: Sha1,
    },
    HashObject {
        object_type: Type,
        file_path: PathBuf,
        write: bool,
    },
    Log {
        commit: Sha1,
        n_logs: u32, // Number of logs to show
    },
    LsTree {
        recursive: bool,
        tree: Sha1,
    },
    Checkout {
        commit: Sha1,
        path: String,
    },
}

pub fn parse_args() -> Result<Command, ParseArgumentsError> {
    let matches = command!()
        .subcommand(
            ClapCommand::new("init").arg(Arg::new("path").value_name("PATH").required(true)),
        )
        .subcommand(
            ClapCommand::new("cat-file")
                .arg(Arg::new("type").value_name("TYPE").required(true))
                .arg(Arg::new("object").value_name("OBJECT").required(true)),
        )
        .subcommand(
            ClapCommand::new("hash-object")
                .about("Compute object ID and optionally creates a blob from a file")
                .arg(
                    Arg::new("write")
                        .short('w')
                        .long("write")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("type")
                        .value_name("TYPE")
                        .short('t')
                        .long("type")
                        .default_value("blob"),
                )
                .arg(Arg::new("file").value_name("FILE").required(true)),
        )
        .subcommand(
            ClapCommand::new("log")
                .about("Display history of given commit")
                .arg(
                    Arg::new("commit")
                        .value_name("COMMIT")
                        .default_value("HEAD")
                        .help("Commit to start at."),
                )
                .arg(
                    Arg::new("n")
                        .value_name("NUMBER")
                        .short('n')
                        .default_value("5")
                        .value_parser(clap::value_parser!(u32))
                        .help("Number of logs to show"),
                ),
        )
        .subcommand(
            ClapCommand::new("ls-tree")
                .about("Pretty-print a tree object.")
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .help("Recurse into sub-trees")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("tree")
                        .value_name("TREE")
                        .help("A tree-ish object"),
                ),
        )
