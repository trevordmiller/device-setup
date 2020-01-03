use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn install_app(app: &str) {
    let installation_status = match Command::new("brew")
        .arg("cask")
        .arg("list")
        .arg(app)
        .output()
    {
        Ok(output) => output.status,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if installation_status.success() {
        println!("The {} app is already installed.", app)
    } else {
        println!("Installing the {} app.", app);
        match Command::new("brew")
            .arg("cask")
            .arg("install")
            .arg(app)
            .output()
        {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn install_package(package: &str) {
    let installation_status = match Command::new("brew").arg("list").arg(package).output() {
        Ok(output) => output.status,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if installation_status.success() {
        println!("The {} package is already installed.", package)
    } else {
        println!("Installing the {} package.", package);
        match Command::new("brew").arg("install").arg(package).output() {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn create_dir(path: &PathBuf) {
    if path.exists() {
        println!("A directory already exists at {}.", path.to_string_lossy())
    } else {
        println!("Creating directory at {}.", path.to_string_lossy());
        match fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn create_file(path: &PathBuf) {
    if path.exists() {
        println!("A file already exists at {}.", path.to_string_lossy())
    } else {
        println!("Creating file at {}.", path.to_string_lossy());
        match fs::File::create(&path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn clone_repo(path: &PathBuf, url: &str) {
        println!("Cloning {} repo.", url);

        match Command::new("git")
            .current_dir(&path)
            .arg("clone")
            .arg(url)
            .output()
        {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
}
