use std::env; // Used to collect arguments from command line
use clap::Parser; // Not currently implemented but will be used in future
use walkdir::WalkDir; // Used to get the contents of folder
use std::io; // Used to read and write files

// Not currently in use, still learning how to use Clap.
#[derive(Parser)]
struct CommandLine {
    commandname: String,
    path: std::path::PathBuf,
}

// An enum for a piece of the JohnnyDecimal tree, that is either a Folder or a File
enum JohnnyTreeMember {
    Folder(String, Vec<JohnnyTreeMember>, String), //
    File(String) 
}

// Implements some constructors for both variants.
impl JohnnyTreeMember {
    fn new_file(path: String) -> JohnnyTreeMember {
        JohnnyTreeMember::File(path)
    }

    fn new_folder(path: String) -> JohnnyTreeMember {
        let init_vector: Vec<JohnnyTreeMember> = Vec::new();
        let code = String::from("A00.00");
        JohnnyTreeMember::Folder(path, init_vector, code)
    }

    fn new_child(mut self, new_member: JohnnyTreeMember) -> self {
        self
    }
}

fn scan() -> Result<JohnnyTreeMember, io::Error> {
    let _ = JohnnyTreeMember::new_file("test".to_string());
    let toplevel = "C:/Users/nateb/JohnnyDecimal"; // Location to start searching from
    let mut areas:Vec<JohnnyTreeMember> = Vec::new(); 
    for entry in WalkDir::new(toplevel).min_depth(1).max_depth(1) {
        let entry = entry?;
        let filename = match entry.file_name().to_str() {
            Some(name) => name,
            None => panic!("Reading error!")
        };
        areas.push(JohnnyTreeMember::new_folder(String::from(filename)));
        // println!("{:?}", filename);    
    }

    for member in areas {
        let mut path_to_member = String::from(toplevel);
        if let JohnnyTreeMember::Folder(filepath, _, _) = member {
            path_to_member = filepath;
            }
        println!("{}", path_to_member);
    }
    
    Ok(JohnnyTreeMember::File("Test".to_string()))
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == String::from("scan") {
        println!("Scanning Johnny Decimal file system...");
        let _ = scan();
    } else if args[1] == String::from("path") {
        println!("Finding path to {}", args[2]);
    }
    
}
