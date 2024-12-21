use std::io::{self, Write};


struct Command;

impl Command{
    
    fn exit(code: i32){
        std::process::exit(code);
    }
    fn not_found(command: &str){
        println!("{}: command not found", command);
    }
}
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


        match input.trim(){
            "exit" => Command::exit(0),
             _ => Command::not_found(input.trim()),
        }
    }
  
}
