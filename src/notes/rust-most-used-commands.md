# Most used commands in Rust

## View documentation

- `rustup doc`

## Scaffold a new project

- `cargo new some_project`

## Compile

- `cargo build`

## Run

- `cargo run`

## Check for errors

- `cargo check`

## Fix syntax

- `cargo fix`

Examples

```sh
# Fix compilation errors.
cargo fix

# Upgrade syntax to a new edition in Cargo.toml
cargo fix --edition
```

## Test logic

- `cargo test`

## Lint for common issues

- `cargo clippy`

## Format source code

- `cargo fmt`

## Generate documentation

- `cargo doc`

Examples:

```sh
# Generate documentation for shipping
cargo doc

# View documentation
cargo doc --open
```

## Share an executable

- `cargo build --release`
- Outputs to `./target/release/some_project`
- Default target is the host architecture
