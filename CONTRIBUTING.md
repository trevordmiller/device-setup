# Contributing

## Setup

Complete the `Setup` steps from the [README.md](./README.md).

## Workflow

- Work off the `master` branch.
- Bump the `version` in `Cargo.toml` following [semver](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field).
- Run `cargo run` to run.
- Run `cargo check` to check for errors.
- Run `cargo fix` to fix syntax.
- Run `cargo test` to test logic.
- Run `cargo clippy` to lint for common issues.
- Run `cargo fmt` to format source code.
- Submit a pull request to the `master` branch.
- Merge the pull request when it has passing merge checks.

## Integrations

- Branch protection with [GitHub settings](https://github.com/trevordmiller/trevordmiller/settings/branches).
- Continuous Integration and Deployment with [GitHub Actions](https://github.com/trevordmiller/trevordmiller/actions).
- Secrets with [GitHub secrets](https://github.com/trevordmiller/trevordmiller/settings/secrets).
