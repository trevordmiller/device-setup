use std::io::ErrorKind;
use std::process::Command;
use structopt::StructOpt;

mod commands;

/// Personal CLI.
#[derive(StructOpt)]
enum Commands {
    /// Reproduce my computer's configuration.
    Setup,

    /// Upgrade what's installed on my computer.
    Upgrade,

    /// Clean up my computer's state.
    End,

    /// Start a study session.
    Study,
}

fn main() {
    match Command::new("which").arg("killall").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("A Unix environment is required."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    };

    match Command::new("brew").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Homebrew is required."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }

    match Commands::from_args() {
        Commands::Setup => commands::setup(),
        Commands::Upgrade => commands::upgrade(),
        Commands::End => commands::end(),
        Commands::Study => commands::study(),
    }

    println!("Finished.");
}
