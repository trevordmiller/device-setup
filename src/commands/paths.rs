use dirs;
use std::fs;
use std::path::PathBuf;

pub fn create_dir(path: &PathBuf, after_create: &dyn Fn()) {
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

pub fn create_file(path: &PathBuf, after_create: &dyn Fn()) {
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

pub fn vim_plugins() -> PathBuf {
    let vim_plugins_path = home()
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start");

    return vim_plugins_path;
}

pub fn vim_configuration() -> PathBuf {
    let vim_configuration_path = home().join(".vimrc");

    return vim_configuration_path;
}

pub fn repos() -> PathBuf {
    let repos_path = home().join("repos");

    return repos_path;
}

fn home() -> PathBuf {
    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    return home_path;
}
