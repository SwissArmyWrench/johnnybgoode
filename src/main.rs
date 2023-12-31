use std::{env, process::exit, collections::HashMap, path::PathBuf}; // Used to collect arguments from command line
// use clap::{}; // Not currently implemented but will be used in future
use walkdir::WalkDir; // Used to get the contents of folder
use std::io; // Used to read and write files

// Not currently in use, still learning how to use Clap.
// #[derive(Parser)]
// struct CommandLine {
//     commandname: String,
//     path: std::path::PathBuf,
// }


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
    // println!("{:?}", &folder); // Uncomment for verbosity for debugging
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
    if args[1] == String::from("path") { // Returns a path to a given location code
        let arg = &args[2];
        println!("Finding path to {}...", arg);
        println!("{}", get_path(String::from(arg)).display());
    } else if args[1] == String::from("scan") { // Runs updated scan method
        scan_to_map();
    } else {
        println!("Unknown command. Please try again.");
        exit(1);
    }
    
}
