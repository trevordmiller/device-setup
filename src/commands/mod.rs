use std::fs;
use std::io::ErrorKind;
use std::process::Command;

mod documents;
mod exercises;
mod feeds;
mod git;
mod homebrew;
mod notes;
mod paths;
mod processes;
mod progress;

pub fn setup() {
    // Vim
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
            "set grepprg=rg\\ --vimgrep
            set grepformat=%f:%l:%c:%m",
        ) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        });
    });
    homebrew::install_package("ripgrep", &|| {});

    // Git
    homebrew::install_package("git", &|| {});

    // Rust
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

    // JavaScript
    homebrew::install_package("node", &|| {});
}

pub fn upgrade() {
    // Unix
    homebrew::upgrade_self();

    // Vim
    homebrew::upgrade_package("vim");
    homebrew::upgrade_package("ripgrep");
    git::pull_all(&paths::vim_plugins());

    // Git
    homebrew::upgrade_package("git");

    // Rust
    homebrew::upgrade_package("rustup-init");
    homebrew::upgrade_rust();

    // JavaScript
    homebrew::upgrade_package("node");
}

pub fn end() {
    // Unix
    homebrew::clean_artifacts();

    // Vim
    processes::stop("vim");

    // Git
    git::check_all(&paths::repos());

    // Rust
    processes::stop("rls");

    // JavaScript
    processes::stop("node");
}

pub fn study() {
    feeds::next();
    exercises::next();
    documents::next();
    notes::next();
    website::release();
}
