# rsw
Rust Workshop

## Topics

### Rust Installation 
* rustup, rustc, cargo, linker deps (gcc/build-essential or clang or mscpp)
    
### Rust environment setup
* VS Code - Rust-analyzer + CodeLLDB
* Vim/Neovim - Rust.vim + Syntastic + Rust-analyzer + CoC  

### Project Types
* bin
    * Cargo new, build, run and debug
    * Cargo check and Cargo fmt
    * std:io example - prelude, 
    * Import dependencies. simple example such as rand or serde
    * println! vs eprintln! vs dbg!
        * redirect streams 
            * `./rsw-app > output` for just stdout (> is same as 1>)
            * `./rsw-app 2> errors` for stderr
            * `./rsw-app 1> output 2> errors` for both
            * `./rsw-app > logs 2>&1` 
* lib
    * cargo new --lib, build
    * cargo test
    * unit testing - run and debug
    * intro to modules (mod)
    * multi-file project example.

### Cargo Workspaces 
* bin + lib workspace demo   
* Rebuilding launch configuration
* debugging on vs code

### Using Cargo install to install other rust based cli tools
* Examples: ripgrep, tokei, bat, etc.