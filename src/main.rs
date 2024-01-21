use johnnybgoode::{Command, Config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // Immediately collect arguments for usage
    let config = Config::load();
    Command::run(Command::new(args, config));
}
