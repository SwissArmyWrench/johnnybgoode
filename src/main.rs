use johnnybgoode::{get_path, scan_to_map, Config};
use std::env;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect(); // Immediately collect arguments for usage

    let config = Config {johnnydecimal_home: PathBuf::from("C:/Users/nateb/JohnnyDecimal/M10-19_Programming/M11-Scripting_and_Automation/M11.03-johnnybgoode/johnnybgoode/sandbox/config.yaml"),
                    name_scheme: String::from("DACID")};

    if args.len() == 1 {
        eprintln!("johnnybgoode: Please supply argument(s)")
    } else if args[1] == "path" {
        // Returns a path to a given location code
        let arg = &args[2];
        // println!("Finding path to {}...", arg); // Enable for greater verbosity
        println!("{}", get_path(config, String::from(arg)).display());
    } else if args[1] == "scan" {
        // Runs updated scan method
        scan_to_map(&config.johnnydecimal_home);
    } else if args[1] == "build" {
        johnnybgoode::build_tree(&scan_to_map(&config.johnnydecimal_home));
    } else if args[1] == "export" {
        johnnybgoode::export(
            johnnybgoode::build_tree(&johnnybgoode::scan_to_map(&config.johnnydecimal_home)),
            std::path::PathBuf::from(args[2].to_owned()),
        )
    } else {
        println!("Unknown command {}. Please try again.", args[1]);
        exit(1);
    }
}
