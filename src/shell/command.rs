use std::{
    env, fs,
    path::{self, Path},
    process,
};

use crate::shell::utils::{is_path, run_program};

pub enum Command<'a> {
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    Run(&'a str, Vec<&'a str>),
    Pwd,
    Cd(&'a str),
}

impl<'a> Command<'a> {
    pub fn execute(self) -> Result<(), String> {
        match self {
            Command::Exit(code) => {
                process::exit(code);
            }
            Command::Echo(message) => {
                println!("{}", message.iter().skip(1).cloned().collect::<Vec<&str>>().join(" "));
                Ok(())
            }
            Command::Type(command) => {
                Self::print_type(command);
                Ok(())
            }
            Command::Pwd => {
                println!("{}", env::current_dir().unwrap().display());
                Ok(())
            }
            Command::Cd(path) => {
                if let Err(_) = env::set_current_dir(Path::new(path)) {
                    Err(format!("cd: no such file or directory: {}", path))
                } else {
                    Ok(())
                }
            }
            Command::Run(program, args) => {
                if is_path(program) {
                    run_program(Path::new(program), args);
                    Ok(())
                } else if let Some(path) = Self::find_executable(program) {
                    run_program(&path, args);
                    Ok(())
                } else {
                    Err(format!("{}: command not found", program))
                }
            }
        }
    }

    pub fn from_tokens(tokens: Vec<&'a str>) -> Command<'a> {
        match tokens.first() {
            Some(&"exit") => Command::Exit(tokens.get(1).and_then(|s| s.parse().ok()).unwrap_or(0)),
            Some(&"echo") => Command::Echo(tokens),
            Some(&"type") => Command::Type(tokens.get(1).unwrap_or(&"")),
            Some(&"pwd") => Command::Pwd,
            Some(&"cd") => Command::Cd(tokens.get(1).unwrap_or(&".")),
            Some(path) => Command::Run(path, tokens[1..].to_vec()),
            None => Command::Echo(vec![]),
        }
    }

    fn print_type(command: &str) {
        match command {
            "exit" | "echo" | "type" | "pwd" | "cd" => println!("{command} is a shell builtin"),
            _ => match Self::find_executable(command) {
                Some(path) => println!("{} is {}", command, path.display()),
                None => println!("{}: not found", command),
            },
        }
    }

    fn find_executable(program: &str) -> Option<path::PathBuf> {
        let paths = env::var_os("PATH")?;
        env::split_paths(&paths)
            .map(|dir| dir.join(program))
            .find(|p| fs::metadata(p).is_ok())
    }
}
