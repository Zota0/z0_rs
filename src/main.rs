#![allow(nonstandard_style)]
#![allow(unused_parens)]

use std::{io::Read, fs::File};

struct Path {
    pathname: String,
    args: Option<Vec<String>>,
}

impl Path {
    fn new(pathname: String, args: Option<Vec<String>>) -> Path {
        Path { pathname, args }
    }
}

fn main() {
    let main_path_name = "scripts/main.z0".to_string();
    let main_path = Path::new(main_path_name, None);
    let main_file = ReadFile(main_path);
    match (main_file) {
        Err(_) => return,
        Ok(_) => {},
    }

    for line in main_file.unwrap().lines() {
        let mut command :String = String::default();

        if line.ends_with(";") {
            command = String::default();
            command = line.replace(";", "");
        } else {
            command += line;
        }

        println!("command: {}", command);
    }
}

fn ReadFile(file: Path) -> Result<String, std::io::Error> {
    println!("\n=====> {} <=====", file.pathname);
    println!("Args:{:?}", file.args);

    let opened_file = File::open(file.pathname);
    
    match opened_file {
        Ok(mut file) => {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);
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