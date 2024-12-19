use std::io::{self, Write};

fn set_input(input: &mut String) {

    input.clear();

    print!("$ ");

    if let Err(error) = io::stdout().flush() {
        println!("error: {error}");
    }

    if let Err(error) = io::stdin().read_line(input) {
        println!("error: {error}");
    }
}
fn main() {
    let mut input = String::new();

    loop{

        set_input(&mut input);

        if input.trim() == ""  {continue};

        println!("{}: command not found", input.trim());
    }
  
}
