use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

pub fn run() {
    // Environment (Unix)

    println!("Removing package manager artifacts.");

    match Command::new("brew").arg("cleanup").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `brew` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }

    // Version control (Git)

    println!("Checking the status of each repo.");

    let home_dir = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    let repos_path = &home_dir.join("repos");

    match fs::read_dir(&repos_path) {
        Ok(paths) => {
            for path in paths {
                let path = match &path {
                    Ok(path) => path.path(),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let dirty_check = match Command::new("git")
                    .current_dir(&path)
                    .arg("status")
                    .arg("--porcelain")
                    .output()
                {
                    Ok(output) => output.stdout,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                if !dirty_check.is_empty() {
                    panic!("The {} repo is dirty.", &path.display());
                }

                let unpushed_check = match Command::new("git")
                    .current_dir(&path)
                    .arg("log")
                    .arg("@{u}..")
                    .output()
                {
                    Ok(output) => output.stdout,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                if !unpushed_check.is_empty() {
                    panic!("The {} repo has unpushed commits.", &path.display());
                }
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    // Editor (Vim)

    println!("Quitting editor processes.");

    match Command::new("killall").arg("vim").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `killall` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }

    // Application programming (JavaScript)

    println!("Quitting application programming processes.");

    match Command::new("killall").arg("node").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `killall` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }

    // Systems programming (Rust)

    println!("Quitting systems programming processes.");

    match Command::new("killall").arg("rls").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `killall` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}
