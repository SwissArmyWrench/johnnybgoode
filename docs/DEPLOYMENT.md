# Deploying for development

Johnnybgoode depends on Rust, and Rust's package manager Cargo. The Rust toolchain can be installed at [rustup.rs](https://rustup.rs).

To download a local copy of this repo, navigate to a folder where you'd like to keep it, and run `git clone https://github.com/SwissArmyWrench/johnnybgoode.git`.

Browse through ARCHITECTURE.md to understand how the codebase works. **To use with your own file system**, a few things need to be updated with some file paths in order for this project to run. Edit the `Config` struct literal in line 8 of `src/main.rs` with the path to the top level of your johnny decimal folder. Also provide either "ACID" (12.34 numbering style) or "DACID" (A12.34 numbering style) for the `name_scheme` field. However, **for experimentation and development**, this can be left as default, since it will reach to the `dummydecimal` testing folder structure found in this repo's root folder. NOTE: For these paths to work, commands must be run from the project's root directory. 

Then, to build from source, run `cargo build`. You can find a freshly compiled executable in /target/debug/johnnybgoode.exe. Note that this executable is compiled with the debug rules, meaning that it is not optimized. To compile a release-ready version, use `cargo build --release`, which outputs into target/release/. If you encounter errors with compiling, run `rustup update` to update to the latest version of the toolchain.

During development, you can use `cargo run` to run the code. To specify arguments, place them after a pair of dashes after the cargo command. For example to run and test `johnny path 11.01`, use the command `cargo run -- path 11.01`.

Before `johnnybgoode` can be used in your terminals, it needs to be added to the system's PATH variable. While an automated script will be made for this eventually to ship to users along with the ready-made executable, this is not yet available, so you will be on your own with adding this program to your PATH. Additionally, it is recommended to use a shell alias to shorten the name from `johnnybgoode` to just `johnny` so you can save some keystrokes during your regular usage.
