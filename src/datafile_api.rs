//
// API for getting and setting data on the data file on the filesystem
//

use std::{fs::File, io::Read};

fn ensure_file_exists() -> Result<File, String> {
    match dirs::home_dir() {
        Some(home_dir) => {
            let data_file = home_dir.join(".todo_data");
            if !data_file.exists() {
                match File::create(data_file) {
                    Err(e) => return Err(e.to_string()),
                    Ok(file) => return Ok(file),
                }
            }

            return match File::options().read(true).write(true).open(data_file) {
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
    let mut data_file = ensure_file_exists().expect("Error: could not create data file");

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

pub fn add_todos(todos: &Vec<String>) {
    ensure_file_exists().expect("Error: could not create data file");
    let data_file = dirs::home_dir().unwrap().join(".todo_data");
}
