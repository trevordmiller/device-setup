use dirs;
use std::fs;
use std::process::Command;

pub fn run() {
    // Environment (Unix)

    println!("Removing package manager artifacts.");

    match Command::new("brew").arg("cleanup").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    // Version control (Git)

    println!("Making sure all repos are clean.");

    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    let repos_path = home_path.join("repos");

    match fs::read_dir(&repos_path) {
        Ok(paths) => {
            let mut all_repos_clean = true;

            for path in paths {
                let path = match &path {
                    Ok(path) => path.path(),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let status_check = match Command::new("git")
                    .current_dir(&path)
                    .arg("status")
                    .arg("--porcelain")
                    .output()
                {
                    Ok(output) => output.stdout,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let unpushed_check = match Command::new("git")
                    .current_dir(&path)
                    .arg("log")
                    .arg("@{u}..")
                    .output()
                {
                    Ok(output) => output.stdout,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                if !status_check.is_empty() || !unpushed_check.is_empty() {
                    all_repos_clean = false;
                    eprintln!("A dirty repo was found: {}", &path.display());
                }
            }

            if !all_repos_clean {
                panic!("Repos are dirty.");
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    // Editor (Vim)

    println!("Quitting editor processes.");

    match Command::new("killall").arg("vim").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    // Application programming (JavaScript)

    println!("Quitting application programming processes.");

    match Command::new("killall").arg("node").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    // Systems programming (Rust)

    println!("Quitting systems programming processes.");

    match Command::new("killall").arg("rls").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }
}
