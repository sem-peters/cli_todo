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

            return match File::options().read(read).write(write).truncate(truncate).open(data_file) {
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
    let mut data_file = ensure_file_exists(true, false, false).expect("Error: could not create data file");

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

    let mut data_file = ensure_file_exists(true, true, true).expect("Error: could not create data file");
    match data_file.write(data.as_bytes()) {
        Ok(_) => {
            println!("Successfully added todo's");
        }
        Err(e) => println!("Error: Could not write to file: {}", e.to_string()),
    }
}
