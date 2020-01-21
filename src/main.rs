use crate::utils::printing;
use structopt::StructOpt;

mod commands;
mod utils;

/// Personal CLI.
#[derive(StructOpt)]
enum Commands {
    /// Reproduce my machine's configuration.
    Setup,

    /// Upgrade what's installed on my machine.
    Upgrade,

    /// Clean up my machine's state.
    End,

    /// Start one of my study sessions.
    Study,

    /// Generate the public directory for my website.
    Generate,
}

fn main() {
    match Commands::from_args() {
        Commands::Setup => {
            printing::heading("Reproducing my machine's configuration.");
            commands::setup();
        }
        Commands::Upgrade => {
            printing::heading("Upgrading what's installed on my machine.");
            commands::upgrade();
        }
        Commands::End => {
            printing::heading("Cleaning up my machine's state.");
            commands::end();
        }
        Commands::Study => {
            printing::heading("Starting one of my study sessions.");
            commands::study();
        }
        Commands::Generate => {
            printing::heading("Generate the public directory for my website.");
            commands::generate();
        }
    }
}
