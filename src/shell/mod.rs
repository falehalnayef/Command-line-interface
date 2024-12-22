pub mod command;
pub mod utils;

use command::Command;
use std::io::{self, Write};

pub struct Shell {
    input_buffer: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.prompt();
            if let Some(command) = self.parse_command() {
                if let Err(err) = command.execute() {
                    eprintln!("{err}");
                }
            }
        }
    }

    fn prompt(&mut self) {
        self.input_buffer.clear();
        print!("$ ");
        io::stdout().flush().ok();

        if io::stdin().read_line(&mut self.input_buffer).is_err() {
            eprintln!("Error reading input");
        }
    }

    fn parse_command(&self) -> Option<Command> {
        let input = self.input_buffer.trim();
        if input.is_empty() {
            return None;
        }

        let tokens = input.split_whitespace().collect::<Vec<&str>>();
        Some(Command::from_tokens(tokens))
    }
}
