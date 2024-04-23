# Goalposts for next release

This file contains a summary of what is yet to be done when looking forward to the upcoming release. [Learn more](#how-this-file-works).

## Release Goals for Version 1.1.0

- [ ] Out-of-place file warnings at all levels
- [ ] User-configurable exceptions to those warnings
- [ ] Exporting the index in serialization languages like YAML, TOML, or JSON
- [ ] Fix markdown exporter to pass the linter's style guide
- [ ] Built in system to walk user through first-time setup

## Release Goals for Version 1.0.0

Version 1.0.0 was released on April 24th, 2024, for all major systems on x86 architectures!

- [x] Create contribution guidelines
- [x] Create deployment system and guidelines
- [x] Create usage docs
- [x] Build in customizability for file/folder name parsing
- [x] Build cloneable / copyable dummy JohnnyDecimal setup for testing
- [x] ~~Integrate env variables for Johnny Decimal top level path~~ Setup standard location for config file on various systems
- [x] Integrate customization YAML file

Notes:
Goals for testing on major shells and platforms have been tabled for the time being. Since the additional shell scripting required is simple and would not affect version numbers. It has been tested on Windows and Linux and the UNIX underpinnings of MacOS should pose no problem.

## Goals for Later Versions

- Jump to area or category folder
- `johnny next` command to create next folder in numbering system
- collect data about file size/space usage
- automatically update index file when something changes
- warnings for out-of-place files or folders
- checking/enforcing of per-folder name schemes
- Use environment variables for non-default config file location
- Help text under `johnny help`
- `johnny info` command to provide info like expected config file path.
- Built in setup system

### How this file works

This file is an ongoing work in progress, just like the rest of `johnnybgoode`. Goals for the next release are added here and then checked off when they are accomplished. Goals for later on after the next release are added under the section "Goals for later versions"
