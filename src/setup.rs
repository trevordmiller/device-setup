use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use crate::install_app;
use crate::install_package;

pub fn setup() {
    // Environment (Unix)
    
    install_app::install_app("1password");
    install_package::install_package("ripgrep");

    // Version control (Git)

    install_package::install_package("git");

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

    install_package::install_package("vim");

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

    install_package::install_package("node");

    // Systems programming (Rust)

    install_package::install_package("rustup-init");

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
