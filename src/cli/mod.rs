use std::path::PathBuf;

use anyhow::anyhow;
use clap::{command, Arg, ArgAction, Command as ClapCommand};

use crate::{error::ParseArgumentsError, git_object::Type};

#[derive(Debug)]
pub enum Command {
    Init {
        path: String,
    },
    CatFile {
        object_type: Type,
        object_hash: String,
    },
    HashObject {
        object_type: Type,
        file_path: PathBuf,
        write: bool,
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
    } else {
        Err(anyhow!("Argument parse failed"))?
    }
}
