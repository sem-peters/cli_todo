# cli_todo

Todo app for your terminal
Warning: this is a work in progress and under heavy development.
The api is not stable and data corruption may occur between updates due to changing data structures.

## Usage

```
todo <command> <arguments...>

Options:
-h, --help     Show this help message and exit
-l, --list     List all todos
-c, --create   Create a new todo
-d, --delete   Delete a todo
-f, --finish   Finish a todo

```

![image](https://github.com/sem-peters/cli_todo/assets/57051686/77284313-2c8d-4544-a093-340014bdd67e)


## Installation

### Download a binary
If you're on Windows x86_64, or Linux x86_64, you can download the latest binary from the [GitHub releases](https://github.com/sem-peters/cli_todo/releases).
Rename the binary to 'todo' (or 'todo.exe') and modify your PATH to make sure it can be found from the terminal.

### Build from source (recommended)
Building from source is the recommended way to install. Make sure you have cargo and rust installed.

Linux / MacOS:
1. git clone https://github.com/sem-peters/cli_todo.git
2. cd cli_todo
3. cargo build --release
4. sudo mv target/release/cli_todo /usr/local/bin/todo

Windows:
1. git clone https://github.com/sem-peters/cli_todo.git
2. cd cli_todo
3. cargo build --release

Now, make sure you either add the target\release directory to the PATH or move the binary into a directory that's in your PATH.
