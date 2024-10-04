//
// API for getting and setting data on the data file on the filesystem
//

use std::{
    fs::File,
    io::{Read, Write},
};

fn ensure_file_exists(read: bool, write: bool, truncate: bool) -> Result<File, String> {
    match dirs::home_dir() {
        Some(home_dir) => {
            let data_file = home_dir.join(".todo_data");
            if !data_file.exists() {
                match File::create(data_file.clone()) {
                    Err(e) => return Err(e.to_string()),
                    Ok(_) => {
                        let new_file = File::options()
                            .read(read)
                            .write(write)
                            .truncate(truncate)
                            .open(data_file)
                            .expect(
                            "Error: Was able to create file, but could not access it afterwards.",
                        );
                        return Ok(new_file);
                    }
                }
            }

            return match File::options()
                .read(read)
                .write(write)
                .truncate(truncate)
                .open(data_file)
            {
                Ok(file) => Ok(file),
                Err(e) => return Err(e.to_string()),
            };
        }
        None => {
            return Err("Error: Could not find home directory".to_string());
        }
    }
}

pub fn get_todos() -> Vec<String> {
    let mut data_file =
        ensure_file_exists(true, false, false).expect("Error: could not create data file");

    let mut contents: String = String::new();
    data_file
        .read_to_string(&mut contents)
        .expect("Error: could not read data file");

    let todos: Vec<String> = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    return todos;
}

pub fn add_todos(todos: Vec<String>) {
    let mut data: String = get_todos().join("\n");
    let args_data = todos
        .iter()
        .map(|a| a.to_string())
        .reduce(|a, b| a + "\n" + b.as_str())
        .expect("Error: Could not format writable data to write to the file.");

    data.push('\n');
    data.push_str(args_data.as_str());

    let mut data_file =
        ensure_file_exists(true, true, true).expect("Error: could not create data file");
    match data_file.write(data.as_bytes()) {
        Ok(_) => {}
        Err(e) => println!("Error: Could not write to file: {}", e.to_string()),
    }
}

pub fn delete_todos(todos: Vec<String>) {
    let data = get_todos();
    let mut new_data: Vec<String> = Vec::new();

    for (i, todo) in data.iter().enumerate() {
        if !todos.contains(&i.to_string()) {
            new_data.push(todo.to_string())
        }
    }

    let mut data_file =
        ensure_file_exists(true, true, true).expect("Error: could not create data file");
    match data_file.write(new_data.join("\n").as_bytes()) {
        Ok(_) => {}
        Err(e) => println!("Error: Could not write to file: {}", e.to_string()),
    }
}

pub fn finish_todos(todos: Vec<String>) {
    let data = get_todos();
    let mut new_data: Vec<String> = Vec::new();

    for (i, todo) in data.iter().enumerate() {
        if todos.contains(&i.to_string()) {
            if todo.ends_with("%FINISHED%") {
                new_data.push(todo.to_string().replace("%FINISHED%", "")); // Remove finished mark
            } else {
                new_data.push(todo.to_string() + "%FINISHED%"); // Add finished mark
            }
        } else {
            new_data.push(todo.to_string());
        }
    }

    let mut data_file =
        ensure_file_exists(true, true, true).expect("Error: could not create data file");
    match data_file.write(new_data.join("\n").as_bytes()) {
        Ok(_) => {}
        Err(e) => println!("Error: Could not write to file: {}", e.to_string()),
    }
}

pub fn move_todos(args: Vec<String>) {
    let mut data = get_todos();

    let mut src = 0;
    let mut dest = 0;
    args.get(0).map(|arg| {
        src = arg.parse().expect(format!("Error: {} is not a valid position", arg).as_str());
    }).expect("Error: Please provide a number as the first argument");

    args.get(1).map(|arg| {
        dest = arg.parse().expect(format!("Error: {} is not a valid position", arg).as_str());
    }).expect("Error: Please provide a number as the second argument");

    if src == dest {
        return;
    }

    let src_value = data.get(src).expect("Error: Source position does not exist").clone();

    if dest >= data.len() {
        data.push(src_value);
        data.remove(src);
    } else  {
        data.remove(src);
        data.insert(dest, src_value);
    }


    let mut data_file =
        ensure_file_exists(true, true, true).expect("Error: could not create data file");
    match data_file.write(data.join("\n").as_bytes()) {
        Ok(_) => {}
        Err(e) => println!("Error: Could not write to file: {}", e.to_string()),
    }
}

