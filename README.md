# trevordmiller

Personal CLI.

## Principles

- Use defaults as much as possible so that I can work on other computers without my configuration.
- Encapsulate all configuration so that I can reproduce my machine state on other computers.

## Setup

Assuming a Unix environment is being used (like macOS, Linux, or Windows Subsystem for Linux).

- Install Homebrew with the command for the current environment from [https://brew.sh](https://brew.sh).
- Run `brew install git` to install Git.
- Run `brew install rustup-init && rustup-init` to install Rust.
- Run `mkdir ~/repos` to create my repos directory.
- Run `git clone https://github.com/trevordmiller/trevordmiller.git ~/repos` to clone this repo.
- Run `cd ~/repos/trevordmiller` to move into the repo.
- Run `git config user.name "Trevor D. Miller"` to configure my username for the repo.
- Run `git config user.email "5497885+trevordmiller@users.noreply.github.com"` to configure my email for the repo.
- Run `cargo run -- setup` to setup my remaining machine state.

## Usage

- Run `cargo doc --open` to view documentation.

## Contributing

- Work off the `master` branch.
- Run `cargo build` to compile.
- Run `cargo run` to run.
- Run `cargo check` to check for errors.
- Run `cargo fix` to fix syntax.
- Run `cargo test` to test logic.
- Run `cargo clippy` to lint for common issues.
- Run `cargo fmt` to format source code.
- Run `cargo doc` to generate documentation.
