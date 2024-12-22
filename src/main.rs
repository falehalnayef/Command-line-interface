use std::{
    env, fs,
    io::{self, Write},
    path::{self, Path},
};

enum Command<'a> {
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    NotFound(&'a str),
}

impl<'a> Command<'a> {
    fn execute(self) {
        match self {
            Command::Exit(code) => std::process::exit(code),
            Command::Echo(message) => {
                for m in message.iter().skip(1) {
                    print!("{} ", m);
                }
                println!();
            }

            Command::Type(command) => Command::print_type(command),
            Command::NotFound(command) => {
                eprintln!("{}: command not found", command);
            }
        }
    }

    fn print_type(command: &str) {
        match command {
            "exit" => println!("exit is a shell builtin"),
            "echo" => println!("echo is a shell builtin"),
            "type" => println!("type is a shell builtin"),
            "pwd" => println!("pwd is a shell builtin"),
            "cd" => println!("cd is a shell builtin"),
            command => match Command::find_executable(command) {
                Some(path) => println!("{} is {} ", command, path.display()),
                None => println!("{}: not found", command),
            },
        }
    }

    fn find_executable(command: &str) -> Option<path::PathBuf> {
        if let Ok(paths) = env::var("PATH") {
            for dir in paths.split(":") {
                let potential_path = Path::new(dir).join(command);
                if fs::metadata(&potential_path).is_ok() {
                    return Some(potential_path);
                }
            }
        }
        None
    }
}

fn match_command<'a>(tokens: Vec<&'a str>) -> Command<'a> {
    match tokens[0] {
        "exit" => Command::Exit(tokens.get(1).and_then(|s| s.parse().ok()).unwrap_or(0)),
        "echo" => Command::Echo(tokens),
        "type" => Command::Type(tokens[1]),
        _ => Command::NotFound(tokens[0]),
    }
}

fn tokenizer(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}

fn set_input(input: &mut String) {
    input.clear();

    print!("$ ");
    if let Err(error) = io::stdout().flush() {
        eprintln!("error: {error}");
    }

    if let Err(error) = io::stdin().read_line(input) {
        eprintln!("error: {error}");
    }
}

fn main() {
    let mut input = String::new();

    loop {
        set_input(&mut input);

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let tokens = tokenizer(input);

        let command = match_command(tokens);

        command.execute();
    }
}
