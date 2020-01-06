use std::fs;
use std::path::PathBuf;

pub fn create(path: &PathBuf, after_create: &dyn Fn()) {
    if path.exists() {
        println!("A directory already exists at {}.", path.to_string_lossy())
    } else {
        println!("Creating directory at {}.", path.to_string_lossy());
        match fs::create_dir_all(&path) {
            Ok(_) => after_create(),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}
