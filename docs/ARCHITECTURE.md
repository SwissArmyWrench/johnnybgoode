# Project Architecture of Johnny Decimal

or, What, Where, Why, and How

## Main files

- src/main.rs: Handles interactions with command line
- src/lib.rs: Main functions and codebase for `johnnydecimal`

### src/main.rs

main.rs is a short and simple file. All it does is collect arguments from the command line, read the configuration from the YAML file, and pass all of that into lib.rs which handles all of the real logic.

### src/lib.rs

lib.rs contains all of the heavy lifting for johnnybgoode. Below is a brief overview of each struct, enum, and major function. This exists so that new developers will be able to understand the inner workings of the codebase without too much difficulty.

- struct Config: Contains the configuration for the program, loaded from the config.yaml file.
- struct Command: contains an "intent" field holding a Subcommand enum (below) as well as the current Config. It features a run() method which runs the command using the aforementioned Config.
- enum Subcommand: an enum with a variant for each command within `johnnybgoode` such as `path` or `export`, plus a `NoCommand` and `NonValid` for when the input can't be parsed into any other variant.
- struct JohnnyFolder: Used in the process of building up the file tree to then be exported as markdown. It has a path, a name, a JohnnyLevel (below) indicating the layer of the tree it's in, and a Vec containing its children.
- enum JohnnyLevel: features variants for Root, Area, Category, and Individual, each containing the folder number/code to allow them to be sorted into order.
- fn scan_to_map(): Uses the `WalkDir` crate to find each of the lowest level folders and build a HashMap to look them up by their location codes.
- fn get_path(): uses the HashMap returned by scan_to_map() and a provided location code to return a full path. Accessed on the command line by `johnnybgoode path` and also used internally within the codebase.
- fn extract_location(): Extracts the location code from a path. Used to build the map.
- fn extract_area(): Extracts an area number from a category number.
- fn extract_cat(): Extracts a category number from a full location code.
- build_tree(): Builds up a tree structure of nested JohnnyFolders for use in exporting. Inefficient and ugly code right now, slated for some sprucing up in the future.
- fn export(): Takes in the tree from build_tree() and generates a Markdown file that serves as an index.
