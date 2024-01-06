use std::{collections::HashMap, path::PathBuf, cmp::Ordering}; // Used to collect arguments from command line
use walkdir::WalkDir; // Used to get the contents of folder

#[derive(Clone, PartialEq, Eq)]
pub struct JohnnyFolder {
    path: PathBuf,
    name: String,
    level: JohnnyLevel,
    children: Vec<JohnnyFolder>
}

impl JohnnyFolder {
    fn get_children(&self) -> &Vec<JohnnyFolder> {
        &self.children
    }

    fn get_children_owned(self) -> Vec<JohnnyFolder> {
        self.children
    }


}

impl Ord for JohnnyFolder {
    fn cmp(&self, other:  &Self) -> Ordering {
        let (selfkey, otherkey) = (self.level.get_sorting_key(), other.level.get_sorting_key());
        if selfkey < otherkey {
            return Ordering::Less
        } else if selfkey == otherkey {
            return Ordering::Equal
        } else if selfkey > otherkey {
            return Ordering::Greater
        } else {
            Ordering::Equal
        }
    }   
}

impl PartialOrd for JohnnyFolder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (selfkey, otherkey) = (self.level.get_sorting_key(), other.level.get_sorting_key());
        Some(selfkey.cmp(&otherkey))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum JohnnyLevel {
    Root,
    Area(i32),
    Category(i32),
    Individual(String)
}

impl JohnnyLevel {
    fn get_cat_number(&self) -> i32 {
        match self {
            JohnnyLevel::Root => unreachable!("get_cat_number() cannot be called on JohnnyLevel::Root"),
            JohnnyLevel::Area(_) => unreachable!("get_cat_number() cannot be called on JohnnyLevel::Area"),
            JohnnyLevel::Category(num) => num.to_owned(),
            JohnnyLevel::Individual(code) => extract_cat(code)
        }
    }

    fn get_area_number(&self) -> i32 {
        match self {
            JohnnyLevel::Root => unreachable!("get_area_number() cannot be called on JohnnyLevel::Root"),
            JohnnyLevel::Area(num) => num.to_owned(),
            JohnnyLevel::Category(num) => extract_area(num.to_owned()),
            JohnnyLevel::Individual(code) => extract_area(extract_cat(code))
        }
    }

    fn get_sorting_key(&self) -> i32 {
        match self {
            JohnnyLevel::Root => unreachable!("Cannot sort Root folders"),
            JohnnyLevel::Area(num) => *num,
            JohnnyLevel::Category(num) => *num,
            JohnnyLevel::Individual(loc_code) => {
                let sliceable: &str = &loc_code;
                let key = &sliceable[4..5];
                str::parse::<i32>(&key).expect("Unable to find a number")
            } // returns ID from DAC.ID
        }
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

// Stable UNLESS improperly sorted file exists in a Root, Area, or Category folder. TODO: Implement some behavior for this.
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

fn extract_name(path: &PathBuf) -> String { // Stable
    let name = match path.file_name() {
        Some(name) => name,
        None => panic!("Unable to read folder/location name (parsing folder name from full path)")
    };
    let name = String::from(name.to_string_lossy());
    name
}
#[test]
fn extract_name_test() { // Stable
    let path = PathBuf::from("C:/Users/nateb/JohnnyDecimal/M10-19_Programming/M11-Scripting_and_Automation/M11.03-johnnybgoode");
    assert_eq!(extract_name(&path), "M11.03-johnnybgoode");
}

fn extract_area(catnumber: i32) -> i32 {
    (catnumber - catnumber % 10)/10
}

fn extract_cat(code: &String) -> i32 {
    // let code = code.chars().collect();
    let code: &str = code;
    let digit = &code[1..2];
    let digit_integer = match str::parse::<i32>(digit) {
        Ok(number) => number,
        Err(error) => panic!("Couldn't pull digit from location code \"{1}\": {0}", error, code)
    };
    // println!("{:?}", digit); // Uncomment for added verbosity
    digit_integer
}

#[test]
fn extract_cat_test() {
    let code = String::from("M11.03");
    assert_eq!(extract_cat(&code), i32::from(11));
}


pub fn build_tree(map: &HashMap<String, PathBuf>) -> JohnnyFolder {
    // let map = scan_to_map();

    // build Vec of all individual JohnnyFolders (bottom level, ID of ACID/DACID)
    let mut individuals: Vec<JohnnyFolder> = Vec::new();
    let paths = map.values();
    for path in paths {
        let new = JohnnyFolder {
            path: path.to_owned(),
            level: JohnnyLevel::Individual(extract_location(path)), 
            name: extract_name(&path), 
            children: Vec::new() };
        individuals.push(new);
    }

    let mut categories: Vec<JohnnyFolder> = Vec::new(); // inits vec of categories
    for k in 0..individuals.len() { // iterates over all the individuals
        let mut added = false; // Flag to know if an ID gets filed to a category, or if a new one must be created
        for i in 0..categories.len() { // Loops over the categories looking for the correct one for current individual
            if categories[i].path == individuals[k].path.parent().unwrap() { // if correct is found, insert a clone of the individual
                categories[i].children.push(individuals[k].clone());
                added = true; // set added flag
            }
        }

        if !added { // if no current cat is found, create it
            categories.push(JohnnyFolder {
                path: individuals[k].path.parent().unwrap().to_owned(), // path to cat folder based on id folder's path
                name: extract_name(&individuals[k].path.parent().unwrap().to_owned()), // extracts folder name based on path
                level: JohnnyLevel::Category(individuals[k].level.get_cat_number()), // needs (String, i32) to preserve origin 
                children: Vec::from([individuals[k].clone()])
            
            })
        }
    }
    // at this point in the code all of the individuals have been sorted away into the appropriate categories
    let mut areas: Vec<JohnnyFolder> = Vec::new(); // init vec of areas
    for k in 0..categories.len() {
        let mut added = false;
        for i in 0..areas.len() {
            if areas[i].path == categories[k].path.parent().unwrap() {
                areas[i].children.push(categories[k].clone());
                added = true; // set added flag
            }
        }
        
        if !added {
            areas.push(JohnnyFolder {
                path: categories[k].path.parent().unwrap().to_owned(),
                name: extract_name(&categories[k].path.parent().unwrap().to_owned()),
                level: JohnnyLevel::Area(categories[k].level.get_area_number()), // TODO: Derive this number
                children: vec!(categories[k].clone())
            })
        }
    }
    
    let root = JohnnyFolder {
        path: areas[0].path.parent().unwrap().to_owned(),
        name: String::from("Johnny Decimal Root Folder"),
        level: JohnnyLevel::Root,
        children: areas
    };
    root
}

pub fn export(filepath: PathBuf) {

}