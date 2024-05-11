# Installing for usage with your system

Johnnybgoode only requires a config file which instructs it of a few optional things, and just one mandatory thing: a full path to the top folder of a Johnny Decimal structure. 

Below is a table that explains where to place this folder, depending on your system:

| Platform | Path |
| -------- | --------- |
| Windows | C:\Users\Alice\AppData\Local\SwissArmyWrench\johnnybgoode\config\config.yaml |
| Linux | /home/Alice/.config/johnnybgoode/config.yaml |
| MacOS | /Users/Alice/Library/Application Support/com.SwissArmyWrench.johnnybgoode/config.yaml |

With this config file in place, and the configuration setup as directed in CONFIGURATION.md, you can call johnnybgoode from the command line, either using an alias that corresponds to the executable itself, or by adding it to PATH. This will be automated with an installer script at some point in the future, but for now, this will need to be done manually.

# Deploying for development

Johnnybgoode depends on Rust, and Rust's package manager Cargo. The Rust toolchain can be installed at [rustup.rs](https://rustup.rs).

To download a local copy of this repo, navigate to a folder where you'd like to keep it, and run `git clone https://github.com/SwissArmyWrench/johnnybgoode.git`.

Browse through ARCHITECTURE.md to understand how the codebase works. **To use with your own file system**, a few things need to be updated with some file paths in order for this project to run. The code will look for a `config.yaml` file your system's standard location for config files. This is handled by the `directories` crate. Below is a table showing the expected locations for a given user named "Alice" on each major system:

| Platform | Path |
| -------- | --------- |
| Windows | C:\Users\Alice\AppData\Local\SwissArmyWrench\johnnybgoode\config\config.yaml |
| Linux | /home/Alice/.config/johnnybgoode/config.yaml |
| MacOS | /Users/Alice/Library/Application Support/com.SwissArmyWrench.johnnybgoode/config.yaml |

In the appropriate location, create a file named `config.yaml` (note that this must be yaml with an A, and not yml, at the time of writing). An example is included in the docs folder to explain how to set it up. For development and/or testing purposes, simply give it an absolute path to the "dummydecimal" folder found in the root of this repo, which contains an example structure. The `name_scheme` option must be set to ACID for this structure to be usable. Alternatively, configure this to work properly with your own production structure and make your terminal navigation far more efficient!

Then, to build from source, run `cargo build`. You can find a freshly compiled executable in /target/debug/johnnybgoode.exe. Note that this executable is compiled with the debug rules, meaning that it is not optimized. To compile a release-ready version, use `cargo build --release`, which outputs into target/release/. If you encounter errors with compiling, run `rustup update` to update to the latest version of the toolchain.

During development, you can use `cargo run` to run the code. To specify arguments, place them after a pair of dashes after the cargo command. For example to run and test `johnny path 11.01`, use the command `cargo run -- path 11.01`.

Before `johnnybgoode` can be used in your terminals, it needs to be added to the system's PATH variable. While an automated script will be made for this eventually to ship to users along with the ready-made executable, this is not yet available, so you will be on your own with adding this program to your PATH. Additionally, it is recommended to use a shell alias to shorten the name from `johnnybgoode` to just `johnny` so you can save some keystrokes during your regular usage.
