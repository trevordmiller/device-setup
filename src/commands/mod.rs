use std::fs;
use std::io::ErrorKind;
use std::process::Command;

mod git;
mod homebrew;
mod paths;
mod processes;
mod progress;

pub fn setup() {
    homebrew::install_package("ripgrep", &|| {});

    homebrew::install_package("git", &|| {});

    homebrew::install_package("vim", &|| {
        let vim_plugins_path = paths::vim_plugins();
        let vim_configuration_path = paths::vim_configuration();

        paths::create_dir(&vim_plugins_path, &|| {
            git::clone(&vim_plugins_path, "https://github.com/tpope/vim-sensible");
            git::clone(&vim_plugins_path, "https://github.com/tpope/vim-sleuth");
            git::clone(&vim_plugins_path, "https://github.com/sheerun/vim-polyglot");
            git::clone(&vim_plugins_path, "https://github.com/octref/RootIgnore");
            git::clone(&vim_plugins_path, "https://github.com/dense-analysis/ale");
        });

        paths::create_file(&vim_configuration_path, &|| match fs::write(
            &vim_configuration_path,
            "set grepprg=rg\\ --vimgrep\nset grepformat=%f:%l:%c:%m",
        ) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        });
    });

    homebrew::install_package("node", &|| {});

    homebrew::install_package("rustup-init", &|| {
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
        }
    });
}

pub fn upgrade() {
    homebrew::upgrade_self();

    homebrew::upgrade_package("ripgrep");

    homebrew::upgrade_package("git");

    homebrew::upgrade_package("vim");

    git::pull_all(&paths::vim_plugins());

    homebrew::upgrade_package("node");

    homebrew::upgrade_package("rustup-init");

    homebrew::upgrade_rust();
}

pub fn end() {
    homebrew::clean_artifacts();

    git::check_all(&paths::repos());

    processes::stop("vim");

    processes::stop("node");

    processes::stop("rls");
}

pub fn study() {
    println!("Follow.");
    progress::pause();

    println!("Practice.");
    progress::pause();

    println!("Research.");
    progress::pause();

    println!("Remember.");
    progress::pause();

    println!("Share.");
    progress::pause();
}
