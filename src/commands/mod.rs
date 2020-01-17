use crate::utils::printing;
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

pub fn setup() {
    printing::heading("Reproducing my computer's configuration.");

    // Vim
    homebrew::install_package("vim", &|| {
        let vim_plugins_path = paths::vim_plugins();

        paths::create_dir(&vim_plugins_path, &|| {
            // Enhance defaults
            git::clone(&vim_plugins_path, "https://github.com/tpope/vim-sensible");

            // Enhance languages
            git::clone(&vim_plugins_path, "https://github.com/sheerun/vim-polyglot");

            // Enhance wildignore
            git::clone(&vim_plugins_path, "https://github.com/octref/RootIgnore");

            // Enhance static analysis
            git::clone(&vim_plugins_path, "https://github.com/dense-analysis/ale");
        });
    });

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
    printing::heading("Upgrading what's installed on my computer.");

    // Unix
    homebrew::upgrade_self();

    // Vim
    homebrew::upgrade_package("vim");
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
    printing::heading("Cleaning up my computer's state.");

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
    // Feeds
    printing::heading("Follow feeds so that I'm aware of industry changes.");
    feeds::next();

    // Exercises
    printing::heading("Complete an exercise so that I gain experience solving problems.");
    exercises::next();

    // Documents
    printing::heading("Read a document so that I increase what I know.");
    documents::next();

    // Notes
    printing::heading("Write a note so that I solidify the most important things I've learned.");
    notes::next();
}
