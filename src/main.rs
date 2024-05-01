use command::CommandType;

mod command;
mod datafile_api;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let commands: Vec<String> = args
        .iter()
        .filter(|arg| arg.starts_with("-"))
        .map(|arg| arg.to_owned())
        .collect();
    let command_type: CommandType;
    if commands.len() < 1 {
        command_type = CommandType::Help;
    } else if commands.len() > 1 {
        println!("Error: please provide only one command");
        return;
    } else {
        command_type = CommandType::from(commands[0].as_str());
    }

    let arguments: Vec<String> = args
        .iter()
        .filter(|arg| !arg.starts_with("-"))
        .map(|arg| arg.to_owned())
        .collect();

    command::run(command_type, arguments);
}
