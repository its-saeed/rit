use clap::{command, Arg, Command as ClapCommand};

#[derive(Debug)]
pub enum Command {
    Init { path: String },
}

pub fn parse_args() -> Result<Command, String> {
    let matches = command!()
        .subcommand(
            ClapCommand::new("init").arg(Arg::new("path").value_name("PATH").required(true)),
        )
        .get_matches();

    match matches.subcommand_matches("init") {
        Some(subcommand) => {
            let path = subcommand.get_one::<String>("path").unwrap().clone();
            Ok(Command::Init { path })
        }
        None => Err("Failed to parse".to_string()),
    }
}
