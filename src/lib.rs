use std::{collections::HashMap, path::PathBuf}; // Used to collect arguments from command line
use walkdir::WalkDir; // Used to get the contents of folder


struct JohnnyFolder {
    code: String,
    name: String,
    level: JohnnyLevel,
    children: Vec<JohnnyFolder>
}

enum JohnnyLevel {
    Root,
    Area,
    Category,
    Individual
}

impl JohnnyFolder {
    fn get_children(&self) -> &Vec<JohnnyFolder> {
        &self.children
    }

    fn get_children_owned(self) -> Vec<JohnnyFolder> {
        self.children
    }
}

pub fn scan_to_map() -> HashMap<String, PathBuf> { // Builds and returns HashMap of location codes to paths
    let mut map: HashMap<String, PathBuf> = HashMap::new(); // Inits HashMap to keep key:path pairs in
    for location in WalkDir::new("C:/users/nateb/JohnnyDecimal").min_depth(3).max_depth(3) { // Walks directories to scan contents
        let item = location.unwrap(); // Semi unsafe, unwrapping Result<T, E> without error handling
        let filepath = item.into_path(); // Turns the item into an owned PathBuf
        let loc_code = extract_location(&filepath); // Uses a reference to that path to extract the location code
        map.insert(loc_code, filepath); // Inserts key and path into the HashMap
    } 
    map // Returns the HashMap
}


pub fn get_path(location: String) -> PathBuf { // Finds path for given location code
    let map = scan_to_map(); // Scans the filesystem and builds map
    let path = map.get(&location); // Extracts the given location from the database // TODO: Build handler for None value
    path.unwrap().to_owned() // Unwraps the Option and turns it to a PathBuf, not a reference to one.
}


fn extract_location(path: &PathBuf) -> String { // Derives location code from full path
    let path = path.to_owned(); // Created owned copy of reference
    let folder = match path.file_name() { // Unwraps the Option
        Some(foldername) => foldername,
        None => panic!("Unable to read folder/location name (parsing folder code from full path")
    };
    // println!("{:?}", &folder); // Uncomment for verbosity for debugging
    let folder = String::from(folder.to_string_lossy()); //Roundabout way of OsStr -> String conversion
    String::from(folder[0..6].to_owned()) //Returns first six characters of folder path // TODO: adjust with environment variables
}
#[test]
fn extract_location_test() { // Test that extract_location() parses folder codes correctly
    let path = PathBuf::from("C:/Users/nateb/JohnnyDecimal/M10-19_Programming/M11-Scripting_and_Automation/M11.03-johnnybgoode");
    assert_eq!(extract_location(&path), "M11.03");
}

fn extract_name(path: &PathBuf) -> String {
    let name = match path.file_name() {
        Some(name) => name,
        None => panic!("Unable to read folder/location name (parsing folder name from full path)")
    };
    let name = String::from(name.to_string_lossy());
    name
}
#[test]
fn extract_name_test() {
    let path = PathBuf::from("C:/Users/nateb/JohnnyDecimal/M10-19_Programming/M11-Scripting_and_Automation/M11.03-johnnybgoode");
    assert_eq!(extract_name(&path), "M11.03-johnnybgoode");
}

fn build_tree(map: &HashMap<String, PathBuf>) /* -> JohnnyFolder */ {
    // let map = scan_to_map();

    // build Vec of all individual JohnnyFolders (bottom level, ID of ACID/DACID)
    let mut individuals: Vec<JohnnyFolder> = Vec::new();
    let paths = map.values();
    for path in paths {
        let new = JohnnyFolder {code: extract_location(path), 
                            name: extract_name(&path), 
                            level: JohnnyLevel::Individual, 
                            children: Vec::new() };
        individuals.push(new);
    }

    let mut categories: Vec<JohnnyFolder> = Vec::new();
    for id in individuals {
        let added = false; // Flag to know if an ID gets filed to a category, or if a new one must be created
        
    }
}