use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

mod dir;
mod file;
mod git;
mod package;

pub fn setup() {
    // Environment (Unix)

    package::install("ripgrep");

    // Version control (Git)

    package::install("git");

    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    // Editor (Vim)

    package::install("vim");

    println!("Adding editor configuration.");

    let editor_plugins_path = home_path
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start");

    dir::create(&editor_plugins_path, &|| {
        git::clone(
            &editor_plugins_path,
            "https://github.com/tpope/vim-sensible",
        );
        git::clone(&editor_plugins_path, "https://github.com/tpope/vim-sleuth");
        git::clone(
            &editor_plugins_path,
            "https://github.com/sheerun/vim-polyglot",
        );
        git::clone(&editor_plugins_path, "https://github.com/octref/RootIgnore");
        git::clone(
            &editor_plugins_path,
            "https://github.com/dense-analysis/ale",
        );
    });

    let editor_configuration_path = home_path.join(".vimrc");

    file::create(&editor_configuration_path, &|| match fs::write(
        &editor_configuration_path,
        "set grepprg=rg\\ --vimgrep\nset grepformat=%f:%l:%c:%m",
    ) {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    });

    // Application programming (JavaScript)

    package::install("node");

    // Systems programming (Rust)

    package::install("rustup-init");

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

    package::upgrade("ripgrep");

    // Version control (Git)

    package::upgrade("git");

    // Editor (Vim)

    package::upgrade("vim");

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

    package::upgrade("node");

    // Systems programming (Rust)

    package::upgrade("rustup-init");

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

pub fn study() {
    println!("Follow.");

    println!("Practice.");

    println!("Research.");

    println!("Remember.");

    println!("Share.");
}
