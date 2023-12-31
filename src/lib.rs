use std::{collections::HashMap, path::PathBuf}; // Used to collect arguments from command line
use walkdir::WalkDir; // Used to get the contents of folder

 pub fn scan_to_map() -> HashMap<String, PathBuf> {
    let mut map: HashMap<String, PathBuf> = HashMap::new();
    for location in WalkDir::new("C:/users/nateb/JohnnyDecimal").min_depth(3).max_depth(3) {
        let item = location.unwrap();
        let filepath = item.into_path();
        let loc_code = extract_location(&filepath);
        map.insert(loc_code, filepath);
    } 
    map
}

pub fn get_path(location: String) -> PathBuf {
    let map = scan_to_map();
    let path = map.get(&location);
    path.unwrap().to_owned()
}

pub fn extract_location(path: &PathBuf) -> String {
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
