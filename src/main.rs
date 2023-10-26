use rit::{parse_args, repository::GitRepository, Command};

fn main() {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => GitRepository::create(path).unwrap(),
    };
}
