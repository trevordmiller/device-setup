use std::fs;
use std::path::PathBuf;

pub fn create(path: &PathBuf, after_create: &dyn Fn()) {
    if path.exists() {
        println!("A file already exists at {}.", path.to_string_lossy())
    } else {
        println!("Creating file at {}.", path.to_string_lossy());
        match fs::File::create(&path) {
            Ok(_) => after_create(),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}
