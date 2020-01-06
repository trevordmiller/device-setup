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
    println!("Skim my feeds so that I keep my skills up-to-date with industry changes.");
    // ./resources/feeds.txt
    progress::pause();

    println!("Complete the next incomplete exercise so that I increase my ability to solve problems.");
    // ./resources/exercises/
    // Exercise README.md
    // Red green tests
    // Compare my solution with the example solution
    progress::pause();

    println!("Read books so that I add to what I know.");
    // ./resources/books.txt
    progress::pause();

    println!("Review and update my notes so that I don't forget what I already know.");
    // ./resources/notes/
    // Use permanant routes for each category
    // Keep content encapsulated in markdown files so that the content is portable with minimal integrations
    progress::pause();

    println!("Publish my updates so that I have a record for reference.");
    // If relevant, send an email to the newsletter email list?
    // Hosting: [Github Pages](https://github.com/trevordmiller/trevordmiller.github.io/settings) with the built output from the `develop` branch to the `master` branch.
    // Custom domain name: [Hover](https://www.hover.com) with the `CNAME` file generated during release.
    progress::pause();
}
