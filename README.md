# trevordmiller

Personal CLI.

## Principles

- Focus on timeless skills so that my skills are relevant as long as possible.
- Use defaults as much as possible so that my skills are portable without being tied to custom setups. Strive to only add configuration that enhances defaults. Capture any added configuration so that it can be reproduced on other machines. 
- Keep input encapsulated in text and markdown files so that the input is portable with minimal integrations.
- Use RSS for feeds so that I can avoid noise like ads, comments, and styles. Stick to official and community voted trends so that I learn from unbiased sources.

## Setup

Assuming a Unix environment is being used (like macOS, Linux, or Windows Subsystem for Linux) with [Homebrew](https://brew.sh) installed.

- Run `brew install git` to install Git.
- Run `brew install rustup-init && rustup-init` to install Rust.
- Run `git clone https://github.com/trevordmiller/trevordmiller.git ~/repos/trevordmiller` to clone this repo.
- Run `cd ~/repos/trevordmiller` to move into the repo.
- Run `cargo run --release -- setup` to setup my remaining machine state.

## Usage

- Run `cargo run --release` to list the available commands.
- Run `cargo run --release -- {command}` to run a command.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).
