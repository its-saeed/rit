use anyhow::anyhow;
use clap::{command, Arg, Command as ClapCommand};

use crate::error::ParseArgumentsError;

#[derive(Debug)]
pub enum Command {
    Init { path: String },
}

pub fn parse_args() -> Result<Command, ParseArgumentsError> {
    let matches = command!()
        .subcommand(
            ClapCommand::new("init").arg(Arg::new("path").value_name("PATH").required(true)),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_matches("init") {
        let path = subcommand.get_one::<String>("path").unwrap().clone();
        return Ok(Command::Init { path });
    }

    Err(anyhow!("Argument parse failed"))?
}
