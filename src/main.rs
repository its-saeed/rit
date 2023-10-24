use rit::parse_args;

fn main() {
    let command = parse_args().unwrap();
    println!("{:?}", command);
}
