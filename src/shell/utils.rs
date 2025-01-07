use std::{
    path::Path,
    process,
};

pub fn is_path(data: &str) -> bool {
    let path = Path::new(data);
    path.exists() && path.is_file()
}

pub fn run_program(path: &Path, args: Vec<String>) {
    match process::Command::new(path).args(args).spawn() {
        Ok(mut child) => {
            child.wait().expect("Command did not finish successfully");
        }
        Err(e) => {
            eprintln!("Error: Failed to execute '{}': {}", path.display(), e)
        }
    }
}
