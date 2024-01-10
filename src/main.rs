use johnnybgoode::{Command, Config};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect(); // Immediately collect arguments for usage

    let config = Config {
        johnnydecimal_home: PathBuf::from("./dummydecimal"),
        name_scheme: String::from("ACID"),
    };
    Command::run(Command::new(args, config));
}
