use johnnybgoode::{get_path, scan_to_map};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == String::from("path") { // Returns a path to a given location code
        let arg = &args[2];
        // println!("Finding path to {}...", arg); // Enable for greater verbosity
        println!("{}", get_path(String::from(arg)).display());
    } else if args[1] == String::from("scan") { // Runs updated scan method
        scan_to_map();
    } else if args[1] == String::from("build") {
        johnnybgoode::build_tree(&scan_to_map());
    } else {
        println!("Unknown command {}. Please try again.", args[1]);
        exit(1);
    }
    
}