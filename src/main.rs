use std::io::ErrorKind;
use std::process::Command;
use structopt::StructOpt;

mod scripts;
mod utils;

/// Personal scripts to automate my computer configuration.
#[derive(StructOpt)]
enum Scripts {
    /// Reproduce my computer's configuration.
    Setup,

    /// Upgrade what's installed on my computer.
    Upgrade,

    /// Clean up my computer's state.
    End,
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

    match Scripts::from_args() {
        Scripts::Setup => scripts::setup(),
        Scripts::Upgrade => scripts::upgrade(),
        Scripts::End => scripts::end(),
    }

    println!("Finished.");
}
