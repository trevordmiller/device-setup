use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use crate::utils;

pub fn setup() {
    // Environment (Unix)
    
    utils::install_app("1password");
    utils::install_package("ripgrep");

    // Version control (Git)

    utils::install_package("git");

    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    let repos_path = home_path.join("repos");

    if repos_path.exists() {
        println!("Repos directory already exists.")
    } else {
        println!("Creating repos directory.");
        match fs::create_dir(&repos_path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }

    match fs::read_dir(&repos_path) {
        Ok(_) => println!("Repos are already installed."),
        Err(_) => {
            println!("Cloning scripts repo.");

            match Command::new("git")
                .current_dir(&repos_path)
                .arg("clone")
                .arg("https://github.com/trevordmiller/scripts")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Cloning study repo.");

            match Command::new("git")
                .current_dir(&repos_path)
                .arg("clone")
                .arg("https://github.com/trevordmiller/study")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Cloning trevordmiller.github.io repo.");

            match Command::new("git")
                .current_dir(&repos_path)
                .arg("clone")
                .arg("https://github.com/trevordmiller/trevordmiller.github.io")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }
        }
    };

    // Editor (Vim)

    utils::install_package("vim");

    let editor_plugins_path = home_path
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start");

    if editor_plugins_path.exists() {
        println!("Editor plugins directory already exists.")
    } else {
        println!("Creating editor plugins directory.");
        match fs::create_dir_all(&editor_plugins_path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }

    let editor_configuration_path = home_path.join(".vimrc");

    if editor_configuration_path.exists() {
        println!("Editor configuration file already exists.")
    } else {
        println!("Creating editor configuration file.");
        match fs::File::create(&editor_configuration_path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }

    match fs::read_dir(&editor_plugins_path) {
        Ok(_) => println!("Editor plugins are already installed."),
        Err(_) => {
            println!("Installing editor plugin to normalize editor defaults.");

            match Command::new("git")
                .current_dir(&editor_plugins_path)
                .arg("clone")
                .arg("https://github.com/tpope/vim-sensible")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Installing editor plugin to enhance editor indentation.");

            match Command::new("git")
                .current_dir(&editor_plugins_path)
                .arg("clone")
                .arg("https://github.com/tpope/vim-sleuth")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Installing editor plugin to enhance editor languages.");

            match Command::new("git")
                .current_dir(&editor_plugins_path)
                .arg("clone")
                .arg("https://github.com/sheerun/vim-polyglot")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Installing editor plugin to enhance editor ignore patterns.");

            match Command::new("git")
                .current_dir(&editor_plugins_path)
                .arg("clone")
                .arg("https://github.com/octref/RootIgnore")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Installing editor plugin to enhance editor static analysis.");

            match Command::new("git")
                .current_dir(&editor_plugins_path)
                .arg("clone")
                .arg("https://github.com/dense-analysis/ale")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Adding editor configuration to enhance editor search.");

            match fs::write(&editor_configuration_path, "set grepprg=rg\\ --vimgrep") {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            match fs::write(&editor_configuration_path, "set grepformat=%f:%l:%c:%m") {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Installing editor plugin to enhance editor colors.");

            match Command::new("git")
                .current_dir(&editor_plugins_path)
                .arg("clone")
                .arg("https://github.com/arcticicestudio/nord-vim")
                .output()
            {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }

            println!("Adding editor configuration to enhance editor colors.");

            match fs::write(editor_configuration_path, "colorscheme nord") {
                Ok(_) => (),
                Err(error) => panic!("There was a problem: {:?}", error),
            }
        }
    };

    // Application programming (JavaScript)

    utils::install_package("node");

    // Systems programming (Rust)

    utils::install_package("rustup-init");

    let rustup_path_check = match Command::new("which")
        .arg("rustup")
        .output()
    {
        Ok(output) => output.stdout,
        Err(error) => panic!("There was a problem: {:?}", error),
    };

    if rustup_path_check.is_empty() {
        match Command::new("rustup-init").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `rustup-init` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    } else {
        println!("rustup-init has already been run.")
    }
}

pub fn upgrade() {
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
