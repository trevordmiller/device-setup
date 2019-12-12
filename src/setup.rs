use dirs;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

pub fn run() {
    // Environment (Unix)

    let password_manager_status = match Command::new("brew")
        .arg("cask")
        .arg("list")
        .arg("1password")
        .output()
    {
        Ok(output) => output.status,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `brew` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    };

    if password_manager_status.success() {
        println!("Password manager is already installed.")
    } else {
        println!("Installing password manager.");
        match Command::new("brew")
            .arg("cask")
            .arg("install")
            .arg("1password")
            .output()
        {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }

    let search_tool_status = match Command::new("brew").arg("list").arg("ripgrep").output() {
        Ok(output) => output.status,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `brew` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    };

    if search_tool_status.success() {
        println!("Search tool is already installed.")
    } else {
        println!("Installing search tool.");
        match Command::new("brew").arg("install").arg("ripgrep").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }

    // Version control (Git)

    let version_control_status = match Command::new("brew").arg("list").arg("git").output() {
        Ok(output) => output.status,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `brew` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    };

    if version_control_status.success() {
        println!("Version control is already installed.")
    } else {
        println!("Installing version control.");
        match Command::new("brew").arg("install").arg("git").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }

    let home_path = match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    };

    let repos_path = home_path.join("repos");

    if repos_path.exists() {
        println!("Repos directory already exists.")
    } else {
        println!("Creating repos directory.");
        match fs::create_dir_all(&repos_path) {
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

    let editor_status = match Command::new("brew").arg("list").arg("vim").output() {
        Ok(output) => output.status,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `brew` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    };

    if editor_status.success() {
        println!("Editor is already installed.")
    } else {
        println!("Installing editor.");
        match Command::new("brew").arg("install").arg("vim").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }

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

    let application_programming_toolchain_status =
        match Command::new("brew").arg("list").arg("node").output() {
            Ok(output) => output.status,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        };

    if application_programming_toolchain_status.success() {
        println!("Application programming toolchain is already installed.")
    } else {
        println!("Installing application programming toolchain.");
        match Command::new("brew").arg("install").arg("node").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }

    // Systems programming (Rust)

    let systems_programming_toolchain_status =
        match Command::new("brew").arg("list").arg("rustup-init").output() {
            Ok(output) => output.status,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        };

    if systems_programming_toolchain_status.success() {
        println!("Systems programming toolchain is already installed.")
    } else {
        println!("Installing systems programming toolchain.");
        match Command::new("brew")
            .arg("install")
            .arg("rustup-init")
            .output()
        {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `brew` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }

        match Command::new("rustup-init").output() {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The `rustup-init` command is missing."),
                other_error => panic!("There was a problem: {:?}", other_error),
            },
        }
    }
}
