use dirs;
use std::fs;
use std::path::PathBuf;
use crate::utils::printing;

pub fn create_dir(path: &PathBuf, after_create: &dyn Fn()) {
    if path.exists() {
        printing::info(format!("A directory already exists at {}.", path.to_string_lossy()))
    } else {
        printing::progress(format!("Creating directory at {}.", path.to_string_lossy()));
        match fs::create_dir_all(&path) {
            Ok(_) => after_create(),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn create_file(path: &PathBuf, after_create: &dyn Fn()) {
    if path.exists() {
        printing::info(format!("A file already exists at {}.", path.to_string_lossy()))
    } else {
        printing::progress(format!("Creating file at {}.", path.to_string_lossy()));
        match fs::File::create(&path) {
            Ok(_) => after_create(),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn vim_plugins() -> PathBuf {
    home()
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start")
}

pub fn vim_configuration() -> PathBuf {
    home().join(".vimrc")
}

pub fn repos() -> PathBuf {
    home().join("repos")
}

fn home() -> PathBuf {
    match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    }
}
