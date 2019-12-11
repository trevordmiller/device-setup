use structopt::StructOpt;

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
    match Scripts::from_args() {
        Scripts::Setup => setup::run(),
        Scripts::Upgrade => upgrade::run(),
        Scripts::End => end::run(),
    }

    println!("Finished.");
}
