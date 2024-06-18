#![allow(nonstandard_style)]
#![allow(unused_parens)]

use std::{io::Read, fs::File};
use core::str::Chars;

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

    for command in commands {
        ParseCommands(command);
    }
}

fn WriteCommands(contents: String) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    let mut last_command: String = String::new();
    let mut brace_level: u32 = 0;

    for line in contents.lines() {
        let current_command: String = line.trim().to_string();

        if current_command.is_empty() {
            continue;
        }

        last_command.push_str(" ");
        last_command.push_str(&current_command);

        
        if current_command.contains('{') {
            brace_level += current_command.matches('{').count() as u32;
        }
        if current_command.contains('}') {
            brace_level -= current_command.matches('}').count() as u32;
        }

        
        if brace_level == 0 && (current_command.ends_with(';') || current_command.ends_with('}')) {
            commands.push(last_command.trim().to_string());
            last_command = String::new();
        }
    }

    
    if !last_command.is_empty() {
        commands.push(last_command.trim().to_string());
    }

    println!("{:#?}", commands);
    return commands;
}

fn ParseCommands(command: String) {

    let mut command = command.trim();

    if command.starts_with("func") {
        let command = command.replace("func", "");
        let mut func_name :Vec<String> = Vec::new();

        let c_ch :Chars = command.chars();
        for f_n in c_ch {
            if f_n == '{' {
                break;
            } else {
                func_name.push(f_n.to_string());
            }
        }

        func_name = func_name.join("").split(" ").map(|x| x.to_string()).collect();
        let func_name = func_name.join("").replace(")", "").replace("(", "");

        return;
    }

    if command.starts_with("??") {
        let command = command.replace("??", "");
        let mut comparison :Vec<String> = Vec::new();

        let c_ch :Chars = command.chars();
        for c_o in c_ch {
            if c_o == '{' {
                break;
            } else {
                comparison.push(c_o.to_string());
            }
        }

        comparison = comparison.join("").split(" ").map(|x| x.to_string()).collect();
        let comparison = comparison.join("").replace(")", "").replace("(", "");

        return;
    }
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