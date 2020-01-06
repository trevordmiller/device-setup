use crate::utils;
use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

pub fn setup() {
    // Environment (Unix)

    utils::install_package("ripgrep");

    // Version control (Git)

    utils::install_package("git");

    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    // Editor (Vim)

    utils::install_package("vim");

    let editor_plugins_path = home_path
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start");

    utils::create_dir(&editor_plugins_path);

    let editor_configuration_path = home_path.join(".vimrc");

    utils::create_file(&editor_configuration_path);

    match fs::read_dir(&editor_plugins_path) {
        Ok(_) => println!("The editor plugins are already installed."),
        Err(_) => {
            utils::clone_repo(&editor_plugins_path, "https://github.com/tpope/vim-sensible");
            utils::clone_repo(&editor_plugins_path, "https://github.com/tpope/vim-sleuth");
            utils::clone_repo(&editor_plugins_path, "https://github.com/sheerun/vim-polyglot");
            utils::clone_repo(&editor_plugins_path, "https://github.com/octref/RootIgnore");
            utils::clone_repo(&editor_plugins_path, "https://github.com/dense-analysis/ale");

            println!("Adding editor configuration.");
            match fs::write(&editor_configuration_path, "set grepprg=rg\\ --vimgrep\nset grepformat=%f:%l:%c:%m") {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }
        }
    };

    // Application programming (JavaScript)

    utils::install_package("node");

    // Systems programming (Rust)

    utils::install_package("rustup-init");

    let rustup_path_check = match Command::new("which").arg("rustup").output() {
        Ok(output) => output.stdout,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if rustup_path_check.is_empty() {
        match Command::new("rustup-init").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The rustup-init command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    } else {
        println!("The rustup-init has already been run.")
    }
}

pub fn upgrade() {
    // Environment (Unix)

    println!("Upgrading package manager.");

    match Command::new("brew").arg("update").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    utils::upgrade_package("ripgrep");

    // Version control (Git)

    utils::upgrade_package("git");

    // Editor (Vim)

    utils::upgrade_package("vim");

    println!("Upgrading editor plugins.");

    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    let editor_plugins_path = home_path
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start");

    match fs::read_dir(&editor_plugins_path) {
        Ok(paths) => {
            for path in paths {
                match Command::new("git")
                    .current_dir(match path {
                        Ok(path) => path.path(),
                        Err(error) => panic!("There was a problem: {:?}", error),
                    })
                    .arg("pull")
                    .output()
                {
                    Ok(_) => (),
                    Err(error) => panic!("There was a problem: {:?}", error),
                }
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    // Application programming (JavaScript)

    utils::upgrade_package("node");

    // Systems programming (Rust)

    utils::upgrade_package("rustup-init");

    println!("Upgrading systems programming toolchain.");

    match Command::new("rustup").arg("update").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The rustup command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}

pub fn end() {
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
