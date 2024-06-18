#![allow(nonstandard_style)]
#![allow(unused_parens)]

use std::{io::Read, fs::File};

struct Path {
    pathname: String,
    args: Option<Vec<String>>,
}

impl Path {
    fn new(pathname: String, args: Option<Vec<String>>) -> Self {
        Path { pathname, args }
    }
}

fn main() {
    let main_path_name = "scripts/main.z0".to_string();
    let main_path = Path::new(main_path_name, None);
    let main_file = ReadFile(main_path);
    let mut contents :String = String::new();

    match main_file {
        Err(_) => return,
        Ok(file_contents) => {
            contents = file_contents;
        },
    }
    
    let commands :Vec<String> = WriteCommands(contents);

    ParseCommand(commands);

}

fn WriteCommands(contents: String) -> Vec<String> {

    let mut commands :Vec<String> = Vec::new();
    let mut last_command :String = String::new();

    for line in contents.lines() {
        
        let mut current_command :String = line.trim().to_string();

        if current_command.is_empty() {
            continue;
        }
        
        if current_command.ends_with(";") {
            current_command = current_command.replace(";", "").to_string();
        } else {
            last_command += &current_command.to_string(); 
        }

        if last_command.ends_with(";") || last_command.ends_with("}") {
            commands.push(last_command.clone());
            last_command = String::new();
        } else {
            commands.push(current_command.clone());
        }
    }

    return commands;
}

fn ParseCommand(command: Vec<String>) {
    println!("Command: {:#?}", command);
}

fn ReadFile(file: Path) -> Result<String, std::io::Error> {
    println!("\n=====> {} <=====", file.pathname);
    println!("Args:{:?}", file.args);

    let opened_file = File::open(file.pathname);

    match opened_file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("==============================");
            Ok(contents)
        },
        Err(e) => {
            println!("Error: {}", e);
            println!("==============================");
            Err(e)
        }
    }
}