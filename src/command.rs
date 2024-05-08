use crate::datafile_api::{self};

pub enum CommandType {
    Help,
    ListTodo,
    AddTodo,
    DeleteTodo,
    FinishTodo,
    Unknown,
}

impl CommandType {
    pub fn from(arg: &str) -> CommandType {
        return match arg {
            "-h" => CommandType::Help,
            "--help" => CommandType::Help,
            "-l" => CommandType::ListTodo,
            "--list" => CommandType::ListTodo,
            "-c" => CommandType::AddTodo,
            "--create" => CommandType::AddTodo,
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
  -c, --create   Create a new todo
  -d, --delete   Delete a todo
  -f, --finish   Finish a todo

Examples:
  todo -l
  todo -c 'Buy milk'
  todo -d 1
  todo -f 1

";
    println!("{}", help_msg);
}

fn run_list_todo() {
    let todos = datafile_api::get_todos();

    let mut res = String::from("Todos:\n\n");

    let index_width = todos.len().to_string().chars().count();
    for (i, todo) in todos.iter().enumerate() {
        let todo_display_value = todo.clone().replace("%FINISHED%", "");
        let finished_indicator = match todo.contains("%FINISHED") {
            true => '\u{2705}',
            false => '\u{274C}',
        };

        res.push_str(format!("{:index_width$} {}\t{}\n", i, finished_indicator, todo_display_value).as_str());
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

fn run_delete_todos(args: Vec<String>) {
    if args.len() < 1 {
        println!("Error: Please provide one or multiple numbers by which to delete");
        return;
    }

    datafile_api::delete_todos(args);
}

fn run_finish_todos(args: Vec<String>) {
    if args.len() < 1 {
        println!("Error: Please provide one or multiple numbers for todos to finish");
        return;
    }

    datafile_api::finish_todos(args);
}

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
