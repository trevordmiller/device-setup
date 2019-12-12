use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

pub fn run() {
    // Environment (Unix)

    println!("Upgrading package manager.");

    match Command::new("brew").arg("update").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    println!("Upgrading password manager.");

    match Command::new("brew")
        .arg("cask")
        .arg("upgrade")
        .arg("1password")
        .output()
    {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    println!("Upgrading search tool.");

    match Command::new("brew").arg("upgrade").arg("ripgrep").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    // Version control (Git)

    println!("Upgrading version control.");

    match Command::new("brew").arg("upgrade").arg("git").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    // Editor (Vim)

    println!("Upgrading editor.");

    match Command::new("brew").arg("upgrade").arg("vim").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

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

    println!("Upgrading application programming toolchain.");

    match Command::new("brew").arg("upgrade").arg("node").output() {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    // Systems programming (Rust)

    println!("Upgrading systems programming toolchain.");

    match Command::new("brew")
        .arg("upgrade")
        .arg("rustup-init")
        .output()
    {
        Ok(_) => (),
        Err(error) => panic!("There was a problem: {:?}", error),
    }

    match Command::new("rustup").arg("update").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `rustup` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }
}
