use crate::utils::printing;
use structopt::StructOpt;

mod commands;
mod utils;

/// Personal CLI.
#[derive(StructOpt)]
enum Commands {
    /// Reproduce my machine's configuration.
    Setup,

    /// Synchronize my machine's state.
    Clean,

    /// Start one of my study sessions.
    Study,

    /// Build the public directory for my website.
    Generate,
}

fn main() {
    match Commands::from_args() {
        Commands::Setup => {
            printing::heading("Reproducing my machine's configuration.");
            commands::setup();
        }
        Commands::Clean => {
            printing::heading("Synchronizing my machine's state.");
            commands::clean();
        }
        Commands::Study => {
            printing::heading("Starting one of my study sessions.");
            commands::study();
        }
        Commands::Generate => {
            printing::heading("Building the public directory for my website.");
            commands::generate();
        }
    }
}
