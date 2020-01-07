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
    match Commands::from_args() {
        Commands::Setup => commands::setup(),
        Commands::Upgrade => commands::upgrade(),
        Commands::End => commands::end(),
        Commands::Study => commands::study(),
    }

    println!("Finished.");
}
