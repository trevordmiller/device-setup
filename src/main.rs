use structopt::StructOpt;
use std::io::ErrorKind;
use std::process::Command;

mod end;
mod setup;
mod upgrade;

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
            ErrorKind::NotFound => panic!("These scripts are only designed to work in a Unix environment."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    };

    match Command::new("brew").output() {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The `brew` command is missing."),
            other_error => panic!("There was a problem: {:?}", other_error),
        },
    }

    match Scripts::from_args() {
        Scripts::Setup => setup::run(),
        Scripts::Upgrade => upgrade::run(),
        Scripts::End => end::run(),
    }

    println!("Finished.");
}
