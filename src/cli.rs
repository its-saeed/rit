use std::path::PathBuf;

use anyhow::anyhow;
use clap::{command, Arg, ArgAction, Command as ClapCommand};

use crate::{error::ParseArgumentsError, git_object::Type};

type Sha1 = String;

#[derive(Debug)]
pub enum TagSubCommand {
    ListTags,
    CreateTagObject { name: String, object: String },
    CreateLightweightTag { name: String, object: String },
}

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
    ShowRef,
    Tag {
        command: TagSubCommand,
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
        .subcommand(
            ClapCommand::new("checkout")
                .about("Checkout a commit inside a directory")
                .arg(
                    Arg::new("commit")
                        .value_name("COMMIT")
                        .help("The commit or tree to checkout"),
                )
                .arg(
                    Arg::new("path")
                        .value_name("PATH")
                        .help("An EMPTY directory to checkout on"),
                ),
        )
        .subcommand(ClapCommand::new("show-ref").about("List references."))
        .subcommand(
            ClapCommand::new("tag")
                .about("List and create tags")
                .arg(
                    Arg::new("name")
                        .value_name("NAME")
                        .help("The new tag's name"),
                )
                .arg(
                    Arg::new("object")
                        .value_name("OBJECT")
                        .help("The object the new tag will point to"),
                )
                .arg(
                    Arg::new("tag_object")
                        .short('a')
                        .long("add-tag-object")
                        .requires("name")
                        .help("Whether to create a tag object")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_matches("init") {
        let path = subcommand.get_one::<String>("path").unwrap().clone();
        Ok(Command::Init { path })
    } else if let Some(subcommand) = matches.subcommand_matches("cat-file") {
        let object_type: String = subcommand.get_one::<String>("type").unwrap().clone();
        let object_hash = subcommand.get_one::<String>("object").unwrap().clone();
        Ok(Command::CatFile {
            object_type: object_type.parse()?,
            object_hash,
        })
    } else if let Some(subcommand) = matches.subcommand_matches("hash-object") {
        let filename: String = subcommand.get_one::<String>("file").unwrap().clone();
        let object_type = subcommand.get_one::<String>("type").unwrap();
        let write = subcommand.get_flag("write");
        Ok(Command::HashObject {
            file_path: PathBuf::from(filename),
            object_type: object_type.parse()?,
            write,
        })
    } else if let Some(subcommand) = matches.subcommand_matches("log") {
        let commit: String = subcommand.get_one::<String>("commit").unwrap().clone();
        let n_logs: u32 = *subcommand.get_one::<u32>("n").unwrap();
        Ok(Command::Log { commit, n_logs })
    } else if let Some(subcommand) = matches.subcommand_matches("ls-tree") {
        let tree: String = subcommand.get_one::<String>("tree").unwrap().clone();
        let recursive = subcommand.get_flag("recursive");
        Ok(Command::LsTree { tree, recursive })
    } else if let Some(subcommand) = matches.subcommand_matches("checkout") {
        let commit: String = subcommand.get_one::<String>("commit").unwrap().clone();
        let path = subcommand.get_one::<String>("path").unwrap().clone();
        Ok(Command::Checkout { commit, path })
    } else if let Some(_) = matches.subcommand_matches("show-ref") {
        Ok(Command::ShowRef)
    } else if let Some(subcommand) = matches.subcommand_matches("tag") {
        let name = subcommand.get_one::<String>("name");
        let object = subcommand.get_one::<String>("object");
        let add_tag_object = subcommand.get_flag("tag_object");
        let add_lightweight_tag = add_tag_object == false && name.is_some();

        if add_tag_object {
            Ok(Command::Tag {
                command: TagSubCommand::CreateTagObject {
                    name: name.unwrap().clone(), // Safe to call unwrap, we specified that if -a presents, name must too.
                    object: object.map(|val| val.clone()).unwrap_or("HEAD".to_string()),
                },
            })
        } else if add_lightweight_tag {
            Ok(Command::Tag {
                command: TagSubCommand::CreateLightweightTag {
                    name: name.unwrap().clone(), // Safe to call unwrap, add_lightweight_tag has a check for presence of name
                    object: object.map(|val| val.clone()).unwrap_or("HEAD".to_string()),
                },
            })
        } else {
            Ok(Command::Tag {
                command: TagSubCommand::ListTags,
            })
        }
    } else {
        Err(anyhow!("Argument parse failed"))?
    }
}
