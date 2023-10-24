use clap::{command, Arg, Command as ClapCommand};

#[derive(Debug)]
pub enum Command {
    Init,
}

pub fn parse_args() -> Result<Command, String> {
    let matches = command!()
        .subcommand(ClapCommand::new("init").arg(Arg::new("init")))
        .get_matches();

    match matches.subcommand_matches("init") {
        Some(_subcommand) => Ok(Command::Init),
        None => Err("Failed to parse".to_string()),
    }
}
