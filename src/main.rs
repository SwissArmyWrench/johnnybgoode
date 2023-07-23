use std::env;
use clap::Parser;
use walkdir::WalkDir;
use std::io;

#[derive(Parser)]
struct CommandLine {
    commandname: String,
    path: std::path::PathBuf,
}

fn scan() -> Result<(), io::Error> {
    for entry in WalkDir::new("C:/Users/nateb/JohnnyDecimal").min_depth(3).max_depth(3) {
        println!("{}", entry?.path().display());
    }
    Ok(())    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world! {}", args[1]);

    if args[1] == String::from("scan") {
        println!("Scanning Johnny Decimal file system...");
        let _ = scan();
    }
    
}
