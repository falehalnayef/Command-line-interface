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
        let mut tokens = Vec::new();
        let mut start = None;
        let mut in_quotes = false;
        let mut quote_char = '\0';
    
        for (i, c) in input.char_indices() {
            match c {
                '\'' | '"' if !in_quotes => {
                    in_quotes = true;
                    quote_char = c;
                    start = Some(i + 1);
                }
                '\'' | '"' if in_quotes && c == quote_char => {
                    in_quotes = false;
                    if let Some(start_index) = start {
                        tokens.push(&input[start_index..i]);
                    }
                    start = None;
                }
                ' ' if !in_quotes => {
                    if let Some(start_index) = start {
                        tokens.push(&input[start_index..i]);
                    }
                    start = None;
                }
                _ if start.is_none() && !in_quotes => {
                    start = Some(i);
                }
                _ => {}
            }
        }
    
        // Add the last token if it exists
        if let Some(start_index) = start {
            tokens.push(&input[start_index..]);
        }
    
        Some(Command::from_tokens(tokens))
}
}
