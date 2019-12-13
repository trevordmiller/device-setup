use structopt::StructOpt;
use std::io::ErrorKind;
use std::process::Command;

mod end;
mod setup;
mod upgrade;
mod install_app;
mod install_package;

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
        Scripts::Setup => setup::setup(),
        Scripts::Upgrade => upgrade::upgrade(),
        Scripts::End => end::end(),
    }

    println!("Finished.");
}
