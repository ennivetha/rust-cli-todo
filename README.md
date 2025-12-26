# Rust CLI Todo App
A simple persistent command-line todo list application built in Rust.


## Features
- Add, list, complete and delete tasks
- Tasks saved to tasks.json (ignored in git)
- Modular code with unit tests and integration tests
- Uses Clap for CLI and Serde for JSON


## Usage
```bash
cargo run -- add "Buy Groceries"
cargo run -- list
cargo run -- complete 1
cargo run -- delete 1
cargo run -- --help