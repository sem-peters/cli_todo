use crate::datafile_api::{self, get_todos};

// -h, --help     Show this help message and exit
// -l, --list     List all todos
// -a, --add      Add a new todo
// -d, --delete   Delete a todo
// -f, --finish   Finish a todo
pub enum CommandType {
    Help,
    ListTodo,
    AddTodo,
    DeleteTodo,
    FinishTodo,
    Unknown,
}

pub struct Command {
    command_type: CommandType,
    args: Vec<String>,
}

impl CommandType {
    pub fn from(arg: &str) -> CommandType {
        return match arg {
            "-h" => CommandType::Help,
            "--help" => CommandType::Help,
            "-l" => CommandType::ListTodo,
            "--list" => CommandType::ListTodo,
            "-a" => CommandType::AddTodo,
            "--add" => CommandType::AddTodo,
            "-d" => CommandType::DeleteTodo,
            "--delete" => CommandType::DeleteTodo,
            "-f" => CommandType::FinishTodo,
            "--finish" => CommandType::FinishTodo,
            _ => CommandType::Unknown,
        };
    }
}

fn run_help() {
    let help_msg = "
Todo - A CLI Todo App

Usage: todo [command] [arguments]

Commands:
  -h, --help     Show this help message and exit
  -l, --list     List all todos
  -a, --add      Add a new todo
  -d, --delete   Delete a todo
  -f, --finish   Finish a todo

Examples:
  todo -l
  todo -a 'Buy milk'
  todo -d 1
  todo -f 1

";
    println!("{}", help_msg);
}

fn run_list_todo() {
    let todos = datafile_api::get_todos();

    let mut res = String::from("Todos:\n"); 

    for (i, todo) in todos.iter().enumerate() {
        res.push_str(format!("{} {}\n", i, todo).as_str());
    }

    res.push_str("\n");

    println!("{}", res);
}

fn run_add_todo(args: Vec<String>) {
    if args.len() < 1 {
        println!("Error: Please provide a todo");
        return;
    }

    datafile_api::add_todos(args);
}

fn run_delete_todos(args: Vec<String>) {}

fn run_finish_todos(args: Vec<String>) {}

pub fn run(command_type: CommandType, args: Vec<String>) {
    match command_type {
        CommandType::Help => {
            run_help();
        }
        CommandType::ListTodo => {
            run_list_todo();
        }
        CommandType::AddTodo => {
            run_add_todo(args);
        }
        CommandType::DeleteTodo => {
            run_delete_todos(args);
        }
        CommandType::FinishTodo => {
            run_finish_todos(args);
        }

        _ => {
            println!("Error: Invalid command");
            println!("Use -h or --help for help");
        }
    }
}
