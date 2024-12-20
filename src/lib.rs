//################################################
//######           Use Statements           ######
//################################################

use directories::ProjectDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use serde_json::Value;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::Write,
    num::ParseIntError,
    path::{Path, PathBuf},
}; // Used to collect arguments from command line
use walkdir::WalkDir; // Used to get the contents of folder

//################################################
//######           Data Modeling            ######
//################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub johnnydecimal_home: PathBuf,
    pub name_scheme: String,
    pub regex: Option<String>,
}

impl Config {
    pub fn load() -> Config {
        let dirs = ProjectDirs::from("com", "SwissArmyWrench", "johnnybgoode")
            .expect("Unable to find config directory");
        let dir = dirs.config_local_dir();
        let mut path = dir.to_path_buf();
        path.push("config.yaml");
        let conf = File::open(path).expect("Unable to open file.");
        let mut config: Config = serde_yaml::from_reader(conf).expect("Unable to parse YAML.");
        if config.regex != None {
            match Regex::new(&config.regex.clone().expect("can't unwrap!")) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error JBG-1028: The supplied regex pattern cannot be compiled. The default will be used instead.");
                    {config.regex = None;}
                }
            }
        }
        config
    }
}

pub struct Command {
    intent: Subcommand,
    config: Config,
}


impl Command {
    // Parses out a command line command and arguments into a Command struct instance
    pub fn new(arguments_vec: &[String], config: Config) -> Command {
        let parsed_arg: Subcommand;
        if arguments_vec.len() == 1 {
            parsed_arg = Subcommand::NoCommand;
        } else if arguments_vec[1] == "path" {
            parsed_arg = Subcommand::GetPath(arguments_vec[2].clone());
        } else if arguments_vec[1] == "export" {
            parsed_arg = Subcommand::ExportMd(PathBuf::from(arguments_vec[2].clone()));
        } else {
            parsed_arg = Subcommand::NonValid(
                format!(
                    "johnnybgoode: \"{}\" is not a recognized command",
                    arguments_vec[1]
                )
                .to_owned(),
            );
        }

        Command {
            intent: parsed_arg,
            config,
        }
    }

    pub fn run(command: Command) {
        match command.intent {
            Subcommand::NonValid(msg) => {
                eprintln!("{msg}");
            }
            Subcommand::GetPath(code) => {
                println!("{}", get_path(&command.config, &code).to_string_lossy());
            }
            Subcommand::ExportMd(path) => {
                let map = scan_to_map(&command.config);
                let treetop = build_tree(&command.config, &map);
                export(treetop, path);
            }
            Subcommand::NoCommand => {
                eprintln!("johnnybgoode: Johnnybgoode must be used with additional arguments, not as a standalone command");
            }
        }
    }
}

enum Subcommand {
    NoCommand,
    NonValid(String),
    GetPath(String),
    ExportMd(PathBuf),
}

#[derive(Clone, PartialEq, Eq)]
pub struct JohnnyFolder {
    path: PathBuf,
    name: String,
    level: JohnnyLevel,
    children: Vec<JohnnyFolder>,
}

impl JohnnyFolder {
    fn get_children(&self) -> &Vec<JohnnyFolder> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<JohnnyFolder> {
        &mut self.children
    }

    fn get_children_owned(self) -> Vec<JohnnyFolder> {
        self.children
    }
}

impl Ord for JohnnyFolder {
    fn cmp(&self, other: &Self) -> Ordering {
        let (selfkey, otherkey) = (self.level.get_sorting_key(), other.level.get_sorting_key());
        match selfkey.cmp(&otherkey) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
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
    Individual(String),
}

impl JohnnyLevel {
    fn get_cat_number(&self) -> i32 {
        match self {
            JohnnyLevel::Root => {
                unreachable!("get_cat_number() cannot be called on JohnnyLevel::Root")
            }
            JohnnyLevel::Area(_) => {
                unreachable!("get_cat_number() cannot be called on JohnnyLevel::Area")
            }
            JohnnyLevel::Category(num) => num.to_owned(),
            JohnnyLevel::Individual(code) => extract_cat(code).unwrap(),
        }
    }

    fn get_area_number(&self) -> i32 {
        match self {
            JohnnyLevel::Root => {
                unreachable!("get_area_number() cannot be called on JohnnyLevel::Root")
            }
            JohnnyLevel::Area(num) => num.to_owned(),
            JohnnyLevel::Category(num) => extract_area(num.to_owned()),
            JohnnyLevel::Individual(code) => extract_area(extract_cat(code).unwrap()),
        }
    }
    fn get_sorting_key(&self) -> i32 {
        match self {
            JohnnyLevel::Root => unreachable!("Cannot sort Root folders"),
            JohnnyLevel::Area(num) | JohnnyLevel::Category(num) => *num,
            JohnnyLevel::Individual(loc_code) => {
                // println!("ATTEMPTING TO PARSE {}", &sliceable); // enable for debug verbosity

                let regex = Regex::new(r"[0-9]{2}[ \.]?(?<KEY>[0-9]{2})").unwrap(); // Safe unwrap
                                                                                    // since it's a
                                                                                    // string
                                                                                    // literal
                let caps = regex.captures(loc_code).unwrap(); // need to determine if this is safe
                str::parse::<i32>(&caps["KEY"]).expect("Regex match failed")
            } // returns ID from DAC.ID
        }
    }
}

pub fn scan_to_map(config: &Config) -> HashMap<String, PathBuf> {
    // Builds and returns HashMap of location codes to paths
    let mut map: HashMap<String, PathBuf> = HashMap::new(); // Inits HashMap to keep key:path pairs in
    for location in WalkDir::new(&config.johnnydecimal_home)
        .min_depth(3)
        .max_depth(3)
    {
        // Walks directories to scan contents
        let item = location.unwrap(); // Semi unsafe, unwrapping Result<T, E> without error handling
        let filepath = item.into_path(); // Turns the item into an owned PathBuf
        let loc_code = extract_location(config, &filepath); // Uses a reference to that path to extract the location code
                                                            // convert to updated validate_code which returns bool
        if validate_code(&loc_code) {
            map.insert(loc_code, filepath);
        } else {
            eprintln!(
                "Misplaced file found at \"{}\", gracefully skipping",
                filepath.to_string_lossy()
            );
        }
    }
    map // Returns the HashMap
}

fn graceful_crash(code: u16) {
    // impl some type of lookup here
    let code = code.to_string();
    let err_json = serde_json::from_str::<Value>(include_str!("err_table.json")).unwrap();
    let err_str = err_json.get(&code).unwrap();
    eprintln!("[ FATL ]: Unexpected failure: Error JBG-{code}\n[ INFO: {err_str}");
    // eprintln!("INFO: {}", err_str);
    std::process::exit(0);
}

pub fn get_path(config: &Config, location: &str) -> PathBuf {
    // Finds path for given location code
    let map = scan_to_map(config); // Scans the filesystem and builds map
    let path = map.get(location); // Extracts the given location from the database // TODO: Build handler for None value
                                  // eprintln!("Location: {0}\nPath: {1:?}", &location, &path);
    let unwrapped: &PathBuf;
    match path {
        Some(returned_path) => {
            unwrapped = returned_path;
        }
        None => {
            eprintln!(
                "Johnnybgoode cannot find any folder corresponding to location code \"{}\"",
                location
            );
            graceful_crash(3077);
            unreachable!();
        }
    };
    unwrapped.to_owned() // Unwraps the Option and turns it to a PathBuf, not a reference to one.
}

// Stable UNLESS improperly sorted file exists in a Root, Area, or Category folder. TODO: Implement some behavior for this.
fn extract_location(config: &Config, path: &Path) -> String {
    let regex = match &config.regex {
        Some(pattern) => Regex::new(pattern.as_str()).unwrap(),
        None => Regex::new(r"(?<AC>[0-9]{2})[ \.]?(?<ID>[0-9]{2})").unwrap(), // Create the main regex to match JD
    };
    let regex_out = regex.captures(path.to_str().unwrap()); // Unsafe unwrap, needs handled
    match regex_out { None => {graceful_crash(1001);}, Some(_) => {} };
    let caps = regex_out.unwrap();
    format!("{}.{}", &caps["AC"], &caps["ID"])
}
#[test]
fn extract_location_test() {
    // Test that extract_location() parses folder codes correctly
    let path = PathBuf::from(r"dummydecimal\10-19-Vehicles\12-Planes\12.03-Cessna");
    let config = Config {
        johnnydecimal_home: PathBuf::from("./dummydecimal"),
        name_scheme: String::from("ACID"),
        regex: None,
    };
    assert_eq!(extract_location(&config, &path), "12.03");
}

fn validate_code(code: &str) -> bool {
    let regex = Regex::new(r"(?<AC>[0-9]{2})[ \.]?(?<ID>[0-9]{2})").unwrap(); // Safe unwrap since
                                                                              // this is a literal
                                                                              // that does not
                                                                              // change.
    regex.is_match(code)
}

#[test]
fn validator_test() {
    let good_code = String::from("11.03"); // currently only passes with M11.03
    let bad_code = String::from("BAD");

    assert!(validate_code(&good_code));
    assert!(!(validate_code(&bad_code)));
}

fn extract_name(path: &Path) -> String {
    // Stable
    let Some(name) = path.file_name() else {
        panic!("Unable to read folder/location name (parsing folder name from full path)")
    };
    let name = String::from(name.to_string_lossy());
    name
}
#[test]
fn extract_name_test() {
    // Stable
    let path = PathBuf::from("C:/Users/nateb/JohnnyDecimal/M10-19_Programming/M11-Scripting_and_Automation/M11.03-johnnybgoode");
    assert_eq!(extract_name(&path), "M11.03-johnnybgoode");
}

fn extract_area(catnumber: i32) -> i32 {
    (catnumber - catnumber % 10) / 10
}

fn extract_cat(code: &str) -> Result<i32, ParseIntError> {
    let regex = Regex::new(r"(?<cat>[0-9]{2})[ \.]?[0-9]{2}").unwrap(); // SAFE
    let capture = &regex.captures(code).unwrap()["cat"];
    str::parse::<i32>(capture)
    /*
    // let code = code.chars().collect();
    let code: &str = code;
    let digit = &code[1..3];
    str::parse::<i32>(digit)
    // println!("{:?}", digit); // Uncomment for added verbosity
    */
}

#[test]
fn extract_cat_test() {
    let code = String::from("M11.03");
    assert_eq!(extract_cat(&code).unwrap(), 11);
}

pub fn build_tree(config: &Config, map: &HashMap<String, PathBuf>) -> JohnnyFolder {
    // let map = scan_to_map();

    // build Vec of all individual JohnnyFolders (bottom level, ID of ACID/DACID)
    let mut individuals: Vec<JohnnyFolder> = Vec::new();
    let paths = map.values();
    for path in paths {
        let new = JohnnyFolder {
            path: path.to_owned(),
            level: JohnnyLevel::Individual(extract_location(config, path)),
            name: extract_name(path),
            children: Vec::new(),
        };

        if validate_code(&extract_location(config, path)) {
            individuals.push(new);
        } else {
            eprintln!(
                "Misplaced file found at \"{}\", gracefully skipping",
                path.to_string_lossy()
            );
        }
    }

    let mut categories: Vec<JohnnyFolder> = Vec::new(); // inits vec of categories
    for individual in &mut individuals {
        // iterates over all the individuals
        let mut added = false; // Flag to know if an ID gets filed to a category, or if a new one must be created
        for category in &mut categories {
            // Loops over the categories looking for the correct one for current individual
            if category.path == individual.path.parent().unwrap() {
                // if correct is found, insert a clone of the individual
                category.children.push(individual.clone());
                added = true; // set added flag
            }
        }

        if !added {
            // if no current cat is found, create it
            categories.push(JohnnyFolder {
                path: individual.path.parent().unwrap().to_owned(), // path to cat folder based on id folder's path
                name: extract_name(individual.path.parent().unwrap()), // extracts folder name based on path
                level: JohnnyLevel::Category(individual.level.get_cat_number()), // needs (String, i32) to preserve origin
                children: Vec::from([individual.clone()]),
            });
        }
    }
    // at this point in the code all of the individuals have been sorted away into the appropriate categories
    let mut areas: Vec<JohnnyFolder> = Vec::new(); // init vec of areas
    for category in categories {
        let mut added = false;
        for area in &mut areas {
            if area.path == category.path.parent().unwrap() {
                area.children.push(category.clone());
                added = true; // set added flag
            }
        }

        if !added {
            areas.push(JohnnyFolder {
                path: category.path.parent().unwrap().to_owned(),
                name: extract_name(category.path.parent().unwrap()),
                level: JohnnyLevel::Area(category.level.get_area_number()), // TODO: Derive this number
                children: vec![category.clone()],
            });
        }
    }

    let root = JohnnyFolder {
        path: areas[0].path.parent().unwrap().to_owned(),
        name: String::from("Johnny Decimal Root Folder"),
        level: JohnnyLevel::Root,
        children: areas,
    };
    root
}

pub fn export(root: JohnnyFolder, filepath: PathBuf) {
    let mut markdown = File::create(filepath).unwrap();
    writeln!(markdown, "# Root\n").expect("Unable to write to markdown file");

    let mut areas = root.get_children_owned();
    areas.sort();
    for mut area in areas {
        // looping over AREAS
        writeln!(
            markdown,
            "## Area {0} - {1}\n",
            area.level.get_area_number(),
            area.name
        )
        .expect("Unable to write to markdown file");
        area.children.sort();

        for cat in area.get_children_mut() {
            // looping over CATEGORIES
            writeln!(
                markdown,
                "### Category {0} - {1}\n",
                cat.level.get_cat_number(),
                cat.name
            )
            .expect("Unable to write to markdown file");
            cat.children.sort();

            for id in cat.get_children() {
                writeln!(markdown, "**{}**\n", id.name).expect("Unable to write to markdown file");
            }
        }
    }
}
