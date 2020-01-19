# trevordmiller

Personal CLI.

## Principles

- Focus on timeless skills so that my skills are relevant as long as possible.
- Use defaults as much as possible so that my skills are portable without being tied to custom setups. Strive to only add configuration that enhances defaults. Capture any added configuration so that it can be reproduced on other machines. 
- Keep contents encapsulated in markdown files so that the content is portable with minimal integrations.

## Setup

Assuming a Unix environment is being used (like macOS, Linux, or Windows Subsystem for Linux).

- Launch terminal.
- Install Homebrew with the command for the current environment from [https://brew.sh](https://brew.sh).
- Run `brew install git` to install Git.
- Run `brew install rustup-init && rustup-init` to install Rust.
- Restart terminal.
- Run `mkdir ~/repos` to create my repos directory.
- Run `git clone https://github.com/trevordmiller/trevordmiller.git ~/repos/trevordmiller` to clone this repo.
- Run `cd ~/repos/trevordmiller` to move into the repo.
- Run `git config user.name "Trevor D. Miller"` to configure my username for the repo.
- Run `git config user.email "5497885+trevordmiller@users.noreply.github.com"` to configure my email for the repo.
- Run `cargo run -- setup` to setup my remaining machine state.

## Usage

- Run `cargo run` to list usage details.

## Contributing

- Work off the `master` branch.
- Run `cargo run -- {command}` to run.
- Run `cargo check` to check for errors.
- Run `cargo fix` to fix syntax.
- Run `cargo test` to test logic.
- Run `cargo clippy` to lint for common issues.
- Run `cargo fmt` to format source code.

### Integrations

- Branch protection with [GitHub Settings](https://github.com/trevordmiller/trevordmiller/settings/branches).
- Continuous Integration with [GitHub Actions](https://github.com/trevordmiller/trevordmiller/actions).
- Website generation from `./notes`to `trevordmiller.com` with [GitHub pages](https://github.com/trevordmiller/trevordmiller.github.io/settings).
- Domain name registration with [Hover](https://www.hover.com).
