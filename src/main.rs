use std::io::{self, Write};

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
            
            Command::Type(command) => println!("{}", Command::get_type(command)),
            Command::NotFound(command) => {
                eprintln!("{}: command not found", command);
            }
        }
    }

    fn get_type(command: &str) -> String{

        match command {
            "exit" => "exit is a shell builtin".to_string(),
            "echo" => "echo is a shell builtin".to_string(),
            "type" => "type is a shell builtin".to_string(),
            _ => format!("{}: not found", command),
        }
    }
}

fn match_command<'a>(tokens: Vec<&'a str>) -> Command<'a> {
    if tokens.is_empty() {
        return Command::NotFound("");
    }

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
