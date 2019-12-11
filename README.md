# scripts

Personal scripts to automate my computer configuration.

## Usage

Assuming a Unix environment is being used (like macOS, Linux, or Windows Subsystem for Linux).

- Install [Homebrew](https://brew.sh) for the correct environment.
- TODO

## Contributing

### Workflow

Assuming [git](https://git-scm.com) and [rust](https://www.rust-lang.org) are installed.

- Work off the `master` branch.
- Run `cargo build` to compile.
- Run `cargo run` to run.
- Run `cargo check` to check for errors.
- Run `cargo fix` to fix syntax.
- Run `cargo test` to test logic.
- Run `cargo clippy` to lint for common issues.
- Run `cargo fmt` to format source code.
- Run `cargo doc` to generate documentation.

### Guidelines

- Use defaults as much as possible so that I can work on other computers without my configuration.
- Encapsulate all configuration so that I can reproduce my setup on other computers.
