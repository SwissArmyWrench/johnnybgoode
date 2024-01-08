# Deploying for development

Johnnybgoode depends on Rust, and Rust's package manager Cargo. The Rust toolchain can be installed at [rustup.rs](https://rustup.rs).

To download a local copy of this repo, navigate to a folder where you'd like to keep it, and run `git clone https://github.com/SwissArmyWrench/johnnybgoode.git`
Browse through ARCHITECTURE.md to understand how the codebase works. Then, to build from source, run `cargo build`. You can find a freshly compiled executable in /target/debug/johnnybgoode.exe. Note that this executable is compiled with the debug rules, meaning that it is not optimized. To compile a release-ready version, use `cargo build --release`, which outputs into target/release/. If you encounter errors with compiling, run `rustup update` to update to the latest version of the toolchain.

Before `johnnybgoode` can be used in your terminals, it needs to be added to the system's PATH variable. While an automated script will be made for this eventually to ship to users along with the ready-made executable, this is not yet available, so you will be on your own with adding this program to your PATH. Additionally, it is recommended to use a shell alias to shorten the name from `johnnybgoode` to just `johnny` so you can save some keystrokes during your regular usage.
