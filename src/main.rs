use std::{env, error::Error, process::exit, collections::HashMap, path::{Path, PathBuf}}; // Used to collect arguments from command line
use clap::{Parser, builder::StringValueParser}; // Not currently implemented but will be used in future
use walkdir::WalkDir; // Used to get the contents of folder
use std::io; // Used to read and write files

// Not currently in use, still learning how to use Clap.
#[derive(Parser)]
struct CommandLine {
    commandname: String,
    path: std::path::PathBuf,
}

// An enum for a piece of the JohnnyDecimal tree, that is either a Folder or a File // DEPRECATED
#[derive(Clone)]
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

    fn new_child(mut self, new_member: JohnnyTreeMember) {
        self = match self {
            JohnnyTreeMember::File(_) => self,
            JohnnyTreeMember::Folder(filepath, mut children, code) => {
                children.push(new_member);
                JohnnyTreeMember::Folder(filepath, children, code)
            }
        };
    }

    /* fn new_child_2(&mut self, new_member: JohnnyTreeMember) {
        let path = self.get_path();
        let mut children = self.get_children();
        children.push(new_member);
        let self_updated = JohnnyTreeMember::new_folder(path);

        self = match self {
            JohnnyTreeMember::File(_) => self,
            JohnnyTreeMember::Folder(_, _, _) => &mut self_updated
        }
        


    } */

    fn get_path(&self) -> String {
        match self {
            JohnnyTreeMember::File(filepath) => String::from(filepath),
            JohnnyTreeMember::Folder(filepath, _ , _ ) => String::from(filepath)
        }
    }

    fn get_children(&self) -> Vec<JohnnyTreeMember> {
        let vector: Vec<JohnnyTreeMember>;
        vector = match self {
            JohnnyTreeMember::File(_) => vec![JohnnyTreeMember::new_file(String::from("NULL"))],
            JohnnyTreeMember::Folder(_, children, _) => children.clone()
        };
        vector
    }
}

struct JohnnyItem {
    key: String,
    path: PathBuf
}

impl JohnnyItem {
    fn new(&mut self, key: String, path: PathBuf) {
        self.key = key;
        self.path = path;
    }
}

fn scan_to_map() -> HashMap<String, PathBuf> {
    let mut map: HashMap<String, PathBuf> = HashMap::new();
    for location in WalkDir::new("C:/users/nateb/JohnnyDecimal").min_depth(3).max_depth(3) {
        let item = location.unwrap();
        let filepath = item.into_path();
        let loc_code = extract_location(&filepath);
        map.insert(loc_code, filepath);
        /*let printable = match map.get(loc_code).to_str() {
            Some(name) => name,
            None => panic!("Error reading files")
        };
        println!("{}", printable); */
    } 
    map
}

// fn extract_code(item) -> String {

// }

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
        
    }
    
    Ok(JohnnyTreeMember::File("Test".to_string()))
}

// fn testbench() {
//     let mut startingmember = JohnnyTreeMember::new_folder(String::from("test/subtest"));
//     startingmember.new_child(JohnnyTreeMember::new_file(String::from("test/subtest/subsubtest")));
//     let data = startingmember.get_path();
//     let children = startingmember.get_children();
//     println!("Startingmember path: {}", data);
//     println!("Submember path: {}", children[0].get_path());

// }

fn get_path(location: String) -> PathBuf {
    let map = scan_to_map();
    let path = map.get(&location);
    path.unwrap().to_owned()
}

// Definition and test for extract_location(). Used for pulling a Johnny Decimal ACID/DACID code. TODO: impl environment vars
fn extract_location(path: &PathBuf) -> String {
    let path = path.to_owned();
    let folder = match path.file_name() {
        Some(foldername) => foldername,
        None => panic!("Unable to read folder/location name (parsing folder name from full path")
    };
    println!("{:?}", &folder);
    let folder = String::from(folder.to_string_lossy());
    String::from(folder[0..6].to_owned())
}
#[test]
fn extract_location_test() { // Test that extract_location() parses folder codes correctly
    let path = PathBuf::from("C:/Users/nateb/JohnnyDecimal/M10-19_Programming/M11-Scripting_and_Automation/M11.03-johnnybgoode");
    assert_eq!(extract_location(&path), "M11.03");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == String::from("scan") {  // Runs the scan //TODO: Cache scan results in some way 
        println!("Scanning Johnny Decimal file system...");
        let _ = scan();
    } else if args[1] == String::from("path") { // Returns a path to a given location code
        let arg = &args[2];
        println!("Finding path to {}...", arg);
        let _ = get_path(String::from(arg));
    } else if args[1] == String::from("keys") { // Runs updated scan method
        scan_to_map();
    } else {
        println!("Unknown command. Please try again.");
        exit(1);
    }
    
}
