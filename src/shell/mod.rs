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
        let mut s_iter = input.chars().peekable();

        let mut cur_s = String::new();
    
        let mut tokens = Vec::new();
    
        let mut in_single_quote = false;
    
        let mut in_double_quote = false;
    
        while let Some(c) = s_iter.next() {
    
            if c == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
    
            } else if c == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
    
            } else if c == '\\' && !in_single_quote && !in_double_quote {
    
                let c = s_iter.next().unwrap();
    
                cur_s.push(c);
    
            } else if c == '\\' && in_double_quote {
    
                match s_iter.peek().unwrap() {
    
                    '\\' | '$' | '"' => {
    
                        cur_s.push(s_iter.next().unwrap());
    
                    }
    
                    _ => cur_s.push(c),
    
                };
    
            } else if c == ' ' && !in_single_quote && !in_double_quote {
    
                if !cur_s.is_empty() {
    
                    tokens.push(cur_s);
    
                    cur_s = String::new();
    
                }
    
            } else {
    
                cur_s.push(c);
    
            }
    
        }
    
        if !cur_s.is_empty() {
    
            tokens.push(cur_s);
    
        }
    
        Some(Command::from_tokens(tokens))
    }
    
    


}