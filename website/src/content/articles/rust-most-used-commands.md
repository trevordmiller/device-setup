# My most used Rust commands

## View documentation

```shell
rustup doc
```

## Scaffold a new executable project

```shell
cargo new some_executable
```

## Scaffold a new library project

```shell
cargo new some_library --lib
```

## Compile an executable in development mode

```shell
cargo build
```

## Run a development mode executable

```shell
./target/debug/some_executable
```

## Compile an executable in production mode

```shell
cargo build --release
```

## Run a production mode executable

```shell
./target/release/some_executable
```

## Compile the executable in development mode then run the executable

```shell
cargo run
```

## Compile the executable in production mode then run the executable

```shell
cargo run --release
```

## Check for compilation errors

```shell
cargo check
```

## Get more information on a compilation error

```shell
rustc --explain some_error_id
```

## Fix compilation errors that can be fixed automatically

```shell
cargo fix
```

## Upgrade syntax to a new edition (in Cargo.toml)

```shell
cargo fix --edition
```

## Test logic

```shell
cargo test
```

## Lint for common issues

```shell
cargo clippy
```

## Format source code

```shell
cargo fmt
```

## Generate project documentation

```shell
cargo doc
```

## View project documentation

```shell
cargo doc --open
```

## Share an executable

```shell
cargo build --release
```

- Outputs to `./target/release/some_project`
- Default target is the host architecture

## Setup continuous integration

Cargo can be used for static analysis. For example, with something like the following:

```yaml
on:
  push:
  schedule:
    - cron: "0 0 * * *"

jobs:
  compile:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Compile
        run: cargo check

  test:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Test
        run: cargo test

  lint:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Lint
        run: cargo clippy --all-targets -- -D warnings

  format:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Format
        run: cargo fmt -- --check
```

## Add a dependency

- `some_library = "x.y.z"` manually added in `[dependencies]` section of `Cargo.toml`
- From `crates.io`
- Updates `Cargo.lock` during the next compile (`cargo build`, `cargo run`, `cargo check`, `cargo test`, etc.)

## Upgrade all dependencies to the latest patch versions

```shell
cargo update
```

## Upgrade a dependency to newer minor/major versions

- Manually change in `[dependencies]` section of `Cargo.toml` to a different version
- Updates `Cargo.lock` during the next compile (`cargo build`, `cargo run`, `cargo check`, `cargo test`, etc.)
